import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import vm from 'node:vm';
import { createHash } from 'node:crypto';
import { initSync, BrowserGame, BrowserAudio } from '../dist/web/pkg/zelda_mobile.js';
import { keyAction, padAction, bindControls } from '../web/input.js';
import { AudioOutput } from '../web/audio.js';
import { prepareAssets } from '../web/startup.js';

const prefix = 'zelda-mobile-rust:v1:';
class MemoryStorage {
  values = new Map();
  failWrites = false;
  get length() { return this.values.size; }
  key(index) { return [...this.values.keys()][index] ?? null; }
  getItem(key) { return this.values.get(key) ?? null; }
  setItem(key, value) {
    if (this.failWrites) throw new Error('Quota exceeded');
    this.values.set(key, value);
  }
  removeItem(key) { this.values.delete(key); }
}
// web-sys checks the global object's Window identity before reading Storage.
globalThis.Window = class { static [Symbol.hasInstance](value) { return value === globalThis; } };
globalThis.window = globalThis;
initSync({ module: await readFile(new URL('../dist/web/pkg/zelda_mobile_bg.wasm', import.meta.url)) });

function fresh() {
  globalThis.localStorage = new MemoryStorage();
  return new BrowserGame();
}
function pulse(game, action) {
  game.input('test:tap', action);
  game.input('test:tap', -1);
}
function enter(game, slot = 0) {
  for (let now = 10; now <= 140; now += 10) game.advance(now);
  pulse(game, 4);
  for (let now = 150; now <= 280; now += 10) game.advance(now);
  assert.equal(game.state(), 30, 'save selection');
  for (let i = 0; i < slot; i++) pulse(game, 1);
  pulse(game, 4);
  for (let now = 290; now <= 560; now += 10) game.advance(now);
  assert.equal(game.state(), 1, `playing slot ${slot + 1}`);
}

test('actual Wasm runs the opening and inventory with embedded resources', () => {
  const game = fresh();
  try {
    const title = game.pixels();
    assert.ok(game.preload_assets() > 200);
    assert.deepEqual(game.pixels(), title, 'sprite preparation does not change the title');
    assert.equal(game.state(), 0);
    assert.equal(game.pixels().length, 240 * 320 * 4);
    enter(game);
    const before = game.pixels();
    // Native reference/opening-replay.json at 560 ms; includes the whole map.
    assert.equal(createHash('sha256').update(before).digest('hex'),
      'f7b7dccef46268f0f720ba553704b573e163d9eee10571a9976ae1ee2df9a151',
      'browser opening pixels exactly match the native renderer');
    game.input('touch:movement', 1);
    game.input('touch:action', 4);
    for (let now = 570; now <= 1120; now += 10) game.advance(now);
    game.input('touch:action', -1);
    game.input('touch:movement', -1);
    assert.notDeepEqual(game.pixels(), before, 'movement/action changes the screen');
    pulse(game, 6);
    game.advance(1260);
    assert.equal(game.state(), 1);
    assert.ok(JSON.parse(game.commands()).includes('Return'), 'inventory exposes Return');
    game.back();
    game.advance(1400);
    assert.equal(game.state(), 1);
    assert.deepEqual(JSON.parse(game.warnings()), []);
  } finally { game.free(); }
});

test('Wasm replays original menu navigation', async () => {
  const game = fresh();
  try {
    let now = 0;
    const steps = JSON.parse(await readFile(new URL('../reference/menu-replay.json', import.meta.url)));
    for (const step of steps) {
      const actions = ['Up', 'Down', 'Left', 'Right', 'Primary', 'Secondary', 'Menu', 'SoftLeft', 'SoftRight'];
      if (step.type === 'wait') {
        const end = now + step.ms;
        while (now < end) { now = Math.min(now + 10, end); game.advance(now); }
      } else if (step.type === 'press') pulse(game, actions.indexOf(step.action));
      else if (step.type === 'release') game.clear_input();
      else if (step.type === 'assert_state') assert.equal(game.state(), step.state);
    }
  } finally { game.free(); }
});

test('all three browser slots persist independently and survive reconstruction', () => {
  let game = fresh();
  const storage = globalThis.localStorage;
  const saved = [];
  try {
    for (let slot = 0; slot < 3; slot++) {
      if (slot) { game.free(); game = new BrowserGame(); }
      enter(game, slot);
      game.checkpoint();
      const name = `ZeldaSave${slot ? slot + 1 : ''}`;
      const record = JSON.parse(storage.getItem(prefix + name));
      assert.deepEqual(record.slice(0, 4), [90, 77, 82, 49]);
      assert.ok(record.length > 100);
      saved.push([...record]);
      for (let i = 0; i < saved.length; i++) {
        assert.deepEqual(JSON.parse(storage.getItem(prefix + `ZeldaSave${i ? i + 1 : ''}`)), saved[i]);
      }
    }
    game.free();
    game = new BrowserGame();
    enter(game, 2);
    assert.deepEqual(JSON.parse(game.warnings()), []);
    assert.ok(storage.getItem(prefix + 'ZeldaSettings'));
  } finally { game.free(); }
});

test('unavailable and full storage remain playable with visible warning messages', () => {
  globalThis.localStorage = null;
  let game = new BrowserGame();
  try {
    assert.match(game.warnings(), /only for this session/);
    enter(game);
    game.checkpoint();
  } finally { game.free(); }
  game = fresh();
  try {
    globalThis.localStorage.failWrites = true;
    enter(game);
    game.checkpoint();
    assert.match(game.warnings(), /could not be saved/);
    assert.deepEqual(JSON.parse(game.warnings()), [], 'warnings are consumed');
  } finally { game.free(); }
});

test('damaged browser records are reported without breaking startup', () => {
  globalThis.localStorage = new MemoryStorage();
  globalThis.localStorage.setItem(prefix + 'ZeldaSettings', JSON.stringify([1, 2, 3]));
  globalThis.localStorage.setItem(prefix + 'ZeldaSave', 'not JSON');
  const game = new BrowserGame();
  try { assert.match(game.warnings(), /damaged|could not be loaded/); }
  finally { game.free(); }
});

test('full storage still loads existing saves before reporting write failures', () => {
  let game = fresh();
  try { enter(game); game.checkpoint(); } finally { game.free(); }
  game = new BrowserGame();
  let expected;
  try {
    game.advance(140); pulse(game, 4); game.advance(280);
    expected = game.pixels();
  } finally { game.free(); }
  globalThis.localStorage.failWrites = true;
  game = new BrowserGame();
  try {
    game.advance(140); pulse(game, 4); game.advance(280);
    assert.deepEqual(game.pixels(), expected, 'saved slot is available even when storage is full');
    assert.match(game.warnings(), /cannot accept new saves/);
  } finally { game.free(); }
});

test('Save & exit persists the active adventure and signals the browser host', () => {
  const game = fresh();
  try {
    enter(game);
    pulse(game, 6);
    game.advance(630);
    assert.equal(JSON.parse(game.commands())[1], 'Save & exit');
    pulse(game, 8);
    game.advance(700);
    assert.equal(game.has_quit(), true);
    assert.ok(globalThis.localStorage.getItem(prefix + 'ZeldaSave'));
  } finally { game.free(); }
});

test('input rejects invalid actions and handles shared sources and clearing', () => {
  const game = fresh();
  try {
    assert.throws(() => game.input('invalid', 99));
    game.input('key:Z', 4);
    game.input('touch:1', 4);
    game.input('key:Z', -1);
    game.input('touch:1', -1);
    game.clear_input();
    game.advance(140);
    assert.equal(game.state(), 30);
  } finally { game.free(); }
});

test('the actual Rust synth renders music and packed effects at browser sample rates', async () => {
  const font = await readFile(new URL('../assets/audio/GeneralUser-GS.sf2', import.meta.url));
  for (const rate of [44100, 48000]) {
    const audio = new BrowserAudio(font, rate);
    try {
      audio.command(JSON.stringify([{ type: 'play', data: { id: 1, name: 'Sound/midi/intro.mid', position: 0, loops: -1, volume: 80 } }]));
      const samples = audio.render(4096);
      assert.equal(samples.length, 8192);
      assert.ok(samples.every(Number.isFinite));
      assert.ok(samples.some(value => Math.abs(value) > .00001));
      audio.command(JSON.stringify([{ type: 'stop', data: 1 }]));
      assert.ok(audio.render(128).every(value => value === 0));
      audio.command(JSON.stringify([{ type: 'play', data: { id: 2, name: 'Sound/Effects.wav', position: 0, loops: 1, volume: 100 } }]));
      assert.ok(audio.render(4096).some(value => value !== 0));
      assert.throws(() => audio.render(10000));
    } finally { audio.free(); }
  }
});

test('audio loading retains current music and discards stale effects', () => {
  const output = new AudioOutput(() => {}, () => {});
  output.send([{ type: 'play', data: { id: 4, name: 'track.mid', volume: 50 } }, { type: 'play', data: { id: 5, name: 'effects.wav' } }]);
  output.send([{ type: 'volume', data: [4, 25] }]);
  assert.equal(output.music.data.volume, 25);
  output.send([{ type: 'stop', data: 4 }]);
  assert.equal(output.music, null);
});

function deferred() {
  let resolve, reject;
  const promise = new Promise((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}

function mockBrowserAudio(t, session, { worklet, workerReady, resume } = {}) {
  const events = [];
  const contexts = [];
  const workers = [];
  const nodes = [];
  function install(name, value) {
    const original = Object.getOwnPropertyDescriptor(globalThis, name);
    Object.defineProperty(globalThis, name, { configurable: true, value });
    t.after(() => {
      if (original) Object.defineProperty(globalThis, name, original);
      else delete globalThis[name];
    });
  }
  install('navigator', { audioSession: session });
  install('AudioContext', class {
    constructor() {
      this.state = 'suspended';
      this.sampleRate = 48000;
      this.destination = {};
      this.audioWorklet = { addModule: async () => { events.push('load-worklet'); await worklet?.promise; } };
      contexts.push(this);
    }
    createBuffer(channels, frames, rate) {
      assert.deepEqual([channels, frames, rate], [1, 1, 48000]);
      return {};
    }
    createBufferSource() {
      return {
        connect() { events.push('connect-source'); },
        start() { events.push('start-source'); this.onended(); },
        disconnect() { events.push('disconnect-source'); },
      };
    }
    async resume() { events.push('resume'); await resume?.promise; this.state = 'running'; }
    async suspend() { events.push('suspend'); this.state = 'suspended'; }
    async close() { this.state = 'closed'; }
  });
  install('AudioWorkletNode', class {
    constructor() { nodes.push(this); }
    port = { postMessage() {}, close() {} };
    connect() {}
    disconnect() { events.push('disconnect-worklet'); }
  });
  install('Worker', class {
    constructor() { workers.push(this); }
    postMessage(message) {
      if (message.type === 'init') {
        events.push('init-worker');
        if (!workerReady) queueMicrotask(() => this.onmessage?.({ data: { type: 'ready' } }));
      }
    }
    terminate() { this.terminated = true; }
  });
  install('MessageChannel', class { port1 = { close() {} }; port2 = { close() {} }; });
  t.after(async () => { for (const context of contexts) await context.close(); });
  return { events, contexts, workers, nodes };
}

test('Safari playback is unlocked synchronously within the enable-sound gesture', async t => {
  const { events, contexts } = mockBrowserAudio(t, {
    set type(value) { events.push(`session:${value}`); },
  });
  const warnings = [];
  const output = new AudioOutput(() => {}, message => warnings.push(message));
  await output.setPaused(false);
  const loading = output.setEnabled(true);
  assert.deepEqual(events, ['load-worklet', 'session:playback', 'connect-source', 'start-source', 'disconnect-source', 'resume'],
    'playback and unlocking happen synchronously inside the gesture');
  await loading;
  assert.ok(events.indexOf('resume') < events.indexOf('init-worker'));
  assert.equal(contexts[0].state, 'running');
  assert.equal(output.ready, true);
  assert.deepEqual(warnings, []);
});

test('audio recovers from Safari interruption without overriding mute or pause', async t => {
  const { contexts, events } = mockBrowserAudio(t, { type: 'auto' });
  const output = new AudioOutput(() => {}, () => {});
  await output.setPaused(false);
  await output.setEnabled(true);
  const context = contexts[0];
  context.state = 'interrupted';
  const recovery = output.resume();
  assert.ok(events.includes('resume'), 'resume is requested during the recovery gesture');
  await recovery;
  assert.equal(context.state, 'running');
  assert.equal(contexts.length, 1, 'the synth and context survive interruption');
  await output.setPaused(true);
  events.length = 0;
  await output.resume();
  assert.deepEqual(events, [], 'touching controls while paused cannot unlock audio');
  await output.setPaused(false);
  assert.equal(context.state, 'running');
  await output.setEnabled(false);
  events.length = 0;
  await output.resume();
  assert.deepEqual(events, [], 'a later gesture cannot undo Sound off');
});

test('enabling sound while paused defers playback until the Resume gesture', async t => {
  const session = { type: 'auto' };
  const { contexts, events } = mockBrowserAudio(t, session);
  const output = new AudioOutput(() => {}, () => {});
  await output.setEnabled(true);
  assert.equal(output.ready, true);
  assert.equal(session.type, 'auto');
  assert.equal(contexts[0].state, 'suspended');
  assert.ok(!events.includes('start-source'));
  const resumed = output.setPaused(false);
  assert.equal(session.type, 'playback');
  assert.ok(events.includes('start-source'));
  await resumed;
  assert.equal(contexts[0].state, 'running');
});

test('browsers without a usable Audio Session API still play through Web Audio', async t => {
  const { contexts } = mockBrowserAudio(t, undefined);
  const warnings = [];
  const output = new AudioOutput(() => {}, message => warnings.push(message));
  await output.setPaused(false);
  await output.setEnabled(true);
  assert.equal(output.ready, true);
  assert.equal(contexts[0].state, 'running');
  await output.setPaused(true);
  globalThis.navigator.audioSession = { set type(_) { throw new Error('Unsupported session'); } };
  await output.setPaused(false);
  assert.equal(contexts[0].state, 'running');
  assert.deepEqual(warnings, []);
});

test('unsupported browser audio reports failure without rejecting gameplay', async () => {
  const warnings = [];
  const output = new AudioOutput(() => {}, message => warnings.push(message));
  await output.setEnabled(true);
  assert.equal(output.enabled, false);
  assert.equal(output.loading, null);
  assert.match(warnings[0], /keep playing/);
});

test('preload shares preparation, stays silent, and Play unlocks the prepared context', async t => {
  const workerReady = deferred();
  const session = { type: 'auto' };
  const { contexts, workers, events } = mockBrowserAudio(t, session, { workerReady });
  const output = new AudioOutput(() => {}, () => {});
  const loading = output.preload();
  assert.equal(output.preload(), loading, 'concurrent callers share one operation');
  await new Promise(setImmediate);
  assert.equal(output.ready, false, 'worker initialization is required');
  assert.equal(contexts.length, 1);
  assert.equal(contexts[0].state, 'suspended');
  assert.equal(session.type, 'auto');
  assert.ok(!events.includes('resume') && !events.includes('start-source'));
  workers[0].onmessage({ data: { type: 'ready' } });
  await loading;
  assert.equal(output.ready, true);
  assert.equal(output.enabled, false);
  const started = output.start();
  assert.equal(session.type, 'playback');
  assert.ok(events.includes('start-source'), 'unlock begins before start() yields');
  await started;
  assert.equal(contexts[0].state, 'running');
  assert.equal(contexts.length, 1, 'Play uses the preloaded context');
  await output.dispose();
});

test('Play waits for audio activation and propagates activation failures', async t => {
  const activation = deferred();
  mockBrowserAudio(t, { type: 'auto' }, { resume: activation });
  const output = new AudioOutput(() => {}, () => {});
  await output.preload();
  let started = false;
  const start = output.start().then(() => { started = true; });
  const rejected = assert.rejects(start, /Audio activation denied/);
  await new Promise(setImmediate);
  assert.equal(started, false);
  activation.reject(new Error('Audio activation denied'));
  await rejected;
  assert.equal(started, false);
  await output.dispose();
});

test('worklet load errors reject preload and release resources without a gameplay warning', async t => {
  const worklet = deferred();
  const { contexts, workers } = mockBrowserAudio(t, undefined, { worklet });
  const warnings = [];
  const output = new AudioOutput(() => {}, message => warnings.push(message));
  const rejected = assert.rejects(output.preload(), /Worklet download failed/);
  worklet.reject(new Error('Worklet download failed'));
  await rejected;
  assert.equal(output.ready, false);
  assert.equal(output.loading, null);
  assert.equal(contexts[0].state, 'closed');
  assert.equal(workers.length, 0);
  assert.deepEqual(warnings, []);
});

test('an activation error during a sound retry also cancels pending preparation', async t => {
  const worklet = deferred(), activation = deferred();
  const { contexts, workers } = mockBrowserAudio(t, undefined, { worklet, resume: activation });
  const warnings = [];
  const output = new AudioOutput(() => {}, message => warnings.push(message));
  await output.setPaused(false);
  const enabling = output.setEnabled(true);
  activation.reject(new Error('Audio activation denied'));
  await enabling;
  worklet.resolve();
  await new Promise(setImmediate);
  assert.equal(output.loading, null);
  assert.equal(output.enabled, false);
  assert.equal(contexts[0].state, 'closed');
  assert.equal(workers.length, 0);
  assert.match(warnings[0], /Audio activation denied/);
});

test('sound can recover from a playback failure while the game keeps running', async t => {
  const { contexts, workers } = mockBrowserAudio(t, undefined);
  const warnings = [];
  const output = new AudioOutput(() => {}, message => warnings.push(message));
  await output.preload();
  await output.start();
  workers[0].onmessage({ data: { type: 'error', message: 'Playback failed' } });
  await new Promise(setImmediate);
  assert.equal(output.ready, false);
  assert.equal(output.enabled, false);
  assert.equal(output.paused, false, 'audio cleanup preserves the game pause state');
  await output.setEnabled(true);
  assert.equal(output.ready, true);
  assert.equal(contexts.length, 2);
  assert.equal(contexts[1].state, 'running');
  assert.equal(warnings.length, 1);
  await output.dispose();
});

test('worker download, decoding, script, and processor errors reject preparation', async t => {
  for (const failure of ['Sound download failed (503)', 'Invalid soundfont', 'Worker script failed', 'processor']) {
    await t.test(failure, async t => {
      const { contexts, workers, nodes } = mockBrowserAudio(t, undefined, { workerReady: deferred() });
      const output = new AudioOutput(() => {}, () => {});
      const loading = output.preload();
      const rejected = assert.rejects(loading, failure === 'processor' ? /playback stopped/ : new RegExp(failure.replace(/[()]/g, '\\$&')));
      await new Promise(setImmediate);
      if (failure === 'processor') nodes[0].onprocessorerror();
      else if (failure === 'Worker script failed') workers[0].onerror({ message: failure });
      else workers[0].onmessage({ data: { type: 'error', message: failure } });
      await rejected;
      assert.equal(output.ready, false);
      assert.equal(workers[0].terminated, true);
      assert.equal(contexts[0].state, 'closed');
    });
  }
});

test('audio timeout includes a stalled worklet and late completion cannot revive it', async t => {
  t.mock.timers.enable({ apis: ['setTimeout'] });
  const worklet = deferred();
  const { contexts, workers } = mockBrowserAudio(t, undefined, { worklet });
  const output = new AudioOutput(() => {}, () => {});
  const rejected = assert.rejects(output.preload(), /Sound loading timed out/);
  t.mock.timers.tick(90000);
  await rejected;
  worklet.resolve();
  await new Promise(setImmediate);
  assert.equal(output.ready, false);
  assert.equal(contexts[0].state, 'closed');
  assert.equal(workers.length, 0, 'late worklet completion does not create a worker');
  await output.preload();
  assert.equal(output.ready, true, 'a new preparation can succeed after failure');
  assert.equal(contexts.length, 2);
  await output.dispose();
});

test('unsupported audio rejects mandatory preload', async () => {
  const output = new AudioOutput(() => {}, () => assert.fail('startup must not offer silent gameplay'));
  await assert.rejects(output.preload(), /does not support game audio/);
  assert.equal(output.ready, false);
});

test('startup waits for both graphics and audio, in either completion order', async t => {
  for (const audioFirst of [false, true]) {
    await t.test(audioFirst ? 'audio first' : 'graphics first', async () => {
      const gameLoad = deferred(), graphics = deferred(), audioLoad = deferred();
      const statuses = [];
      const game = { preload_assets: () => graphics.promise, free: () => assert.fail('successful game was freed') };
      let playable = false, audioRequested = false;
      const preparation = prepareAssets({
        loadGame: () => gameLoad.promise,
        audio: { preload: () => { audioRequested = true; return audioLoad.promise; } },
        status: message => statuses.push(message),
      }).then(result => { playable = true; return result; });
      assert.equal(audioRequested, true, 'audio starts while the game is downloading');
      assert.deepEqual(statuses, ['Loading game…']);
      if (audioFirst) audioLoad.resolve();
      gameLoad.resolve(game);
      await new Promise(setImmediate);
      assert.equal(playable, false);
      assert.equal(statuses.at(-1), 'Preparing graphics…');
      graphics.resolve();
      await new Promise(setImmediate);
      assert.equal(playable, audioFirst, 'unfinished audio keeps startup blocked');
      audioLoad.resolve();
      assert.equal(await preparation, game);
      assert.deepEqual(statuses, ['Loading game…', 'Preparing graphics…', 'Preparing audio…']);
    });
  }
});

test('game download, sprite decoding, and audio failures keep startup blocked', async t => {
  for (const stage of ['game', 'graphics', 'audio']) {
    await t.test(stage, async () => {
      let freed = 0, disposed = 0;
      const error = new Error(`${stage} failed`);
      const game = { preload_assets() { if (stage === 'graphics') throw error; }, free() { freed++; } };
      await assert.rejects(prepareAssets({
        loadGame: async () => { if (stage === 'game') throw error; return game; },
        audio: {
          preload: async () => { if (stage === 'audio') throw error; },
          dispose: async () => { disposed++; },
        },
        status() {},
      }), error);
      assert.equal(disposed, 1);
      assert.equal(freed, stage === 'game' ? 0 : 1);
    });
  }
});

test('startup timeout blocks late completion and a fresh retry can succeed', async t => {
  t.mock.timers.enable({ apis: ['setTimeout'] });
  const gameLoad = deferred();
  let disposed = 0, freed = 0;
  const statuses = [];
  const game = { preload_assets() {}, free() { freed++; } };
  const rejected = assert.rejects(prepareAssets({
    loadGame: () => gameLoad.promise,
    audio: { preload: async () => {}, dispose: async () => { disposed++; } },
    status: message => statuses.push(message),
  }), /Loading timed out/);
  t.mock.timers.tick(90000);
  await rejected;
  gameLoad.resolve(game);
  await new Promise(setImmediate);
  assert.equal(disposed, 1);
  assert.equal(freed, 1);
  assert.deepEqual(statuses, ['Loading game…'], 'late assets cannot update the failed screen');
  assert.equal(await prepareAssets({ loadGame: async () => game, audio: { preload: async () => {} }, status() {} }), game);
});

test('D-pad sectors, neutral zone, and keyboard mappings', () => {
  const rect = { left: 10, top: 20, width: 150, height: 150 };
  assert.equal(padAction(85, 25, rect), 0);
  assert.equal(padAction(85, 160, rect), 1);
  assert.equal(padAction(15, 95, rect), 2);
  assert.equal(padAction(155, 95, rect), 3);
  assert.equal(padAction(85, 95, rect), -1);
  assert.equal(padAction(-100, 95, rect), -1);
  assert.equal(keyAction('KeyZ'), 4);
  assert.equal(keyAction('Enter'), 6);
  assert.equal(keyAction('Tab'), undefined);
});

test('touch holds, sliding, pointer cancellation, and keyboard sharing', () => {
  const windowEvents = new EventTarget();
  globalThis.addEventListener = windowEvents.addEventListener.bind(windowEvents);
  class Button extends EventTarget {
    constructor(action) { super(); this.dataset = { action: String(action) }; this.classList = { toggle() {} }; this.disabled = false; }
    closest() { return this; }
    setPointerCapture(id) { this.capture = id; }
    hasPointerCapture(id) { return this.capture === id; }
    releasePointerCapture() { this.capture = null; }
  }
  const buttons = Array.from({ length: 9 }, (_, i) => new Button(i));
  const pad = new Button(-1);
  pad.contains = button => buttons.slice(0, 4).includes(button) || button === pad;
  pad.getBoundingClientRect = () => ({ left: 0, top: 0, width: 150, height: 150 });
  const deck = new EventTarget();
  deck.querySelector = () => pad;
  deck.querySelectorAll = () => buttons;
  const edges = [];
  const control = bindControls(deck, { active: () => true, input: (...edge) => edges.push(edge), back() {} });
  function send(type, target, props) {
    const event = new Event(type, { cancelable: true });
    Object.defineProperty(event, 'target', { value: target });
    Object.assign(event, props);
    deck.dispatchEvent(event);
  }
  send('pointerdown', buttons[0], { pointerId: 1, pointerType: 'touch', clientX: 75, clientY: 10 });
  send('pointerdown', buttons[4], { pointerId: 2, pointerType: 'touch' });
  send('pointermove', pad, { pointerId: 1, clientX: 140, clientY: 75 });
  send('pointercancel', buttons[4], { pointerId: 2 });
  assert.deepEqual(edges, [['pointer:1', 0], ['pointer:2', 4], ['pointer:1', 3], ['pointer:2', -1]]);
  control.clear();
  assert.deepEqual(edges.at(-1), ['pointer:1', -1]);
});

test('audio worklet produces stereo PCM, silence on underrun, and ignores stale buffers', async () => {
  let Processor;
  const sandbox = {
    AudioWorkletProcessor: class { port = {}; }, Float32Array,
    registerProcessor: (_name, processor) => { Processor = processor; },
  };
  vm.runInNewContext(await readFile(new URL('../web/audio-worklet.js', import.meta.url), 'utf8'), sandbox);
  const processor = new Processor();
  const requests = [];
  const source = { postMessage: request => requests.push(request) };
  processor.port.onmessage({ data: { type: 'connect', port: source } });
  assert.equal(requests.length, 3);
  assert.equal(processor.pending, 3072);
  source.onmessage({ data: { generation: 0, samples: new Float32Array([.25, -.25, .5, -.5]) } });
  const left = new Float32Array(128), right = new Float32Array(128);
  processor.process([], [[left, right]]);
  assert.deepEqual([...left.slice(0, 3)], [.25, .5, 0]);
  assert.deepEqual([...right.slice(0, 3)], [-.25, -.5, 0]);
  processor.port.onmessage({ data: { type: 'flush' } });
  source.onmessage({ data: { generation: 0, samples: new Float32Array([1, 1]) } });
  assert.equal(processor.available, 0);
});
