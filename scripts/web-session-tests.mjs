import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createGameSession } from '../web/src/game-session.js';

function browser(t) {
  const sessions = [];
  const restore = [];
  t.after(() => {
    for (const session of sessions) session.dispose();
    for (const reset of restore.reverse()) reset();
  });
  function install(name, value) {
    const previous = Object.getOwnPropertyDescriptor(globalThis, name);
    Object.defineProperty(globalThis, name, { configurable: true, value });
    restore.push(() => {
      if (previous) Object.defineProperty(globalThis, name, previous);
      else delete globalThis[name];
    });
  }
  const window = new EventTarget();
  const document = new EventTarget();
  const frames = new Map();
  let nextFrame = 0;
  let reloads = 0;
  window.location = { reload() { reloads++; } };
  document.hidden = false;
  install('window', window);
  install('document', document);
  install('ImageData', class { data = new Uint8ClampedArray(240 * 320 * 4); });
  install('requestAnimationFrame', callback => { frames.set(++nextFrame, callback); return nextFrame; });
  install('cancelAnimationFrame', id => frames.delete(id));
  class Button extends EventTarget {
    constructor(action) {
      super();
      this.dataset = { action };
      this.classList = { toggle() {} };
    }
    closest() { return this; }
  }
  const buttons = Array.from({ length: 9 }, (_, action) => new Button(action));
  const pad = { contains: () => false };
  const deck = new EventTarget();
  deck.querySelector = () => pad;
  deck.querySelectorAll = () => buttons;
  const changes = [];
  const notices = [];
  let draws = 0;
  return {
    window, document, frames, changes, notices,
    get draws() { return draws; },
    get reloads() { return reloads; },
    session(loadGame) {
      const session = createGameSession({
        canvas: { getContext: () => ({ putImageData() { draws++; } }) },
        deck, loadGame,
        assetBase: new URL('https://example.com/zelda-mobile/'),
        onChange: patch => changes.push(patch),
        onNotice: message => notices.push(message),
      });
      sessions.push(session);
      return session;
    },
    frame(now) {
      const [id, callback] = frames.entries().next().value;
      frames.delete(id);
      callback(now);
    },
    key(type, code) {
      const event = new Event(type, { cancelable: true });
      Object.assign(event, { code });
      window.dispatchEvent(event);
    },
  };
}

function game() {
  return {
    elapsed: [], inputs: [], checkpoints: 0, freed: 0, clears: 0,
    quit: false, labels: ['Select', 'Back'], messages: [],
    advance(now) { this.elapsed.push(now); },
    pixels: () => new Uint8Array(240 * 320 * 4),
    commands() { return JSON.stringify(this.labels); },
    drain_audio: () => '[]',
    warnings() { return JSON.stringify(this.messages.splice(0)); },
    input(...args) { this.inputs.push(args); },
    back() {},
    clear_input() { this.clears++; },
    checkpoint() { this.checkpoints++; },
    has_quit() { return this.quit; },
    free() { this.freed++; },
  };
}

test('session draws Wasm frames, publishes commands, and excludes paused time', async t => {
  const host = browser(t);
  const instance = game();
  instance.messages.push('Storage is read-only.');
  const session = host.session(async () => instance);
  await session.ready;
  assert.equal(host.changes.at(-1).status, 'ready');
  assert.deepEqual(host.changes[0].commands, ['Select', 'Back']);
  assert.equal(host.notices[0], 'Storage is read-only.');
  assert.equal(host.draws, 1);
  session.play();
  host.frame(1000);
  host.frame(1040);
  host.key('keydown', 'ArrowDown');
  host.window.dispatchEvent(new Event('blur'));
  assert.equal(host.changes.at(-1).status, 'paused');
  assert.equal(instance.checkpoints, 1);
  assert.deepEqual(instance.inputs, [['key:ArrowDown', 1], ['key:ArrowDown', -1]]);
  host.frame(9000);
  assert.deepEqual(instance.elapsed, [0, 40]);
  session.play();
  host.frame(10000);
  host.frame(10020);
  assert.deepEqual(instance.elapsed, [0, 40, 40, 60]);
  session.dispose();
  assert.equal(host.frames.size, 0);
  assert.equal(instance.freed, 1);
});

test('StrictMode unmount while Wasm loads frees the late instance without starting a loop', async t => {
  const host = browser(t);
  const instance = game();
  let finishLoading;
  const session = host.session(() => new Promise(resolve => { finishLoading = resolve; }));
  session.dispose();
  finishLoading(instance);
  await session.ready;
  assert.equal(instance.freed, 1);
  assert.equal(instance.checkpoints, 0);
  assert.equal(host.frames.size, 0);
  assert.deepEqual(host.changes, []);
});

test('remount removes old input and lifecycle handlers and checkpoints on navigation', async t => {
  const host = browser(t);
  const firstGame = game();
  const first = host.session(async () => firstGame);
  await first.ready;
  first.play();
  first.dispose();
  const secondGame = game();
  const second = host.session(async () => secondGame);
  await second.ready;
  second.play();
  host.key('keydown', 'KeyZ');
  host.key('keyup', 'KeyZ');
  assert.deepEqual(firstGame.inputs, []);
  assert.deepEqual(secondGame.inputs, [['key:KeyZ', 4], ['key:KeyZ', -1]]);
  host.document.hidden = true;
  host.document.dispatchEvent(new Event('visibilitychange'));
  assert.equal(firstGame.checkpoints, 1);
  assert.equal(secondGame.checkpoints, 1);
  assert.equal(host.frames.size, 1);
  second.dispose();
  second.dispose();
  assert.equal(secondGame.freed, 1);
});

test('saved exit stops advancement and offers a fresh game', async t => {
  const host = browser(t);
  const instance = game();
  const session = host.session(async () => instance);
  await session.ready;
  session.play();
  instance.quit = true;
  host.frame(100);
  assert.equal(host.changes.at(-1).status, 'ended');
  assert.equal(host.changes.at(-1).button, 'Play again');
  host.frame(200);
  assert.deepEqual(instance.elapsed, [0]);
  session.play();
  assert.equal(host.reloads, 1);
});

test('Wasm startup failures expose a reload action without scheduling gameplay', async t => {
  const host = browser(t);
  t.mock.method(console, 'error', () => {});
  const session = host.session(async () => { throw new Error('Wasm could not be loaded.'); });
  await session.ready;
  assert.equal(host.changes.at(-1).status, 'error');
  assert.equal(host.changes.at(-1).message, 'Wasm could not be loaded.');
  assert.equal(host.frames.size, 0);
  session.play();
  assert.equal(host.reloads, 1);
});
