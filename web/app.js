import { bindControls } from './input.js';
import { AudioOutput } from './audio.js';
import { prepareAssets } from './startup.js';

const $ = selector => document.querySelector(selector);
const canvas = $('#game');
const context = canvas.getContext('2d', { alpha: false });
const frame = new ImageData(240, 320);
const overlay = $('#overlay');
const resume = $('#resume');
const sound = $('#sound');
const deck = $('.control-deck');
let game, controls, running = false, ended = false, broken = false;
let startupReady = false, hasStarted = false, starting = false, activation = 0;
let elapsed = 0, last = null, lastCommands = '';

function notice(message) {
  $('#notice-text').textContent = message;
  $('#notice').hidden = false;
}
$('#dismiss').addEventListener('click', () => { $('#notice').hidden = true; });
const audio = new AudioOutput(() => {
  sound.setAttribute('aria-pressed', String(audio.enabled));
  sound.setAttribute('aria-label', audio.enabled ? 'Mute sound' : 'Enable sound');
  sound.setAttribute('aria-busy', String(!!audio.loading && !audio.ready));
  sound.title = audio.loading && !audio.ready ? 'Loading sound…' : sound.getAttribute('aria-label');
}, (message, error) => {
  if (!hasStarted) startupFailure(error || new Error(message));
  else notice(message);
});

function enableControls(enabled) {
  deck.inert = !enabled;
  for (const button of deck.querySelectorAll('[data-action]')) button.disabled = !enabled;
  sound.disabled = !enabled;
}

function showOverlay(title, text, button) {
  $('#overlay-title').textContent = title;
  $('#overlay-text').textContent = text;
  resume.textContent = button;
  resume.hidden = false;
  resume.disabled = false;
  $('#loading').hidden = true;
  overlay.hidden = false;
}
function loading(message) {
  $('#overlay-text').textContent = message;
  resume.hidden = true;
  resume.disabled = true;
  $('#loading').hidden = false;
}
function startupFailure(error) {
  activation++;
  starting = false;
  startupReady = running = false;
  broken = true;
  enableControls(false);
  game?.free();
  game = null;
  void audio.dispose();
  console.error(error);
  showOverlay('Could not load game', String(error.message || error), 'Retry');
}
function fatal(error) {
  broken = true;
  running = false;
  console.error(error);
  void audio.setPaused(true);
  showOverlay('Adventure interrupted', String(error.message || error), 'Reload');
}
function attempt(callback) {
  try { return callback(); } catch (error) { fatal(error); }
}
function readWarnings() {
  const warnings = JSON.parse(game.warnings());
  if (warnings.length) notice(warnings.join(' '));
}
function draw() {
  frame.data.set(game.pixels());
  context.putImageData(frame, 0, 0);
  const commands = game.commands();
  if (commands !== lastCommands) {
    lastCommands = commands;
    const labels = JSON.parse(commands);
    for (const [index, selector] of ['#command-left', '#command-right'].entries()) {
      const button = $(selector);
      button.textContent = labels[index] || (index ? 'Right command' : 'Left command');
      button.disabled = !labels[index];
    }
  }
}
function pause() {
  if (starting) {
    activation++;
    starting = false;
    resume.disabled = false;
    void audio.setPaused(true);
  }
  if (!game || !running) return;
  running = false;
  last = null;
  attempt(() => {
    controls.clear();
    game.clear_input();
    game.checkpoint();
    readWarnings();
    if (!ended && !broken) showOverlay('Game paused', '', 'Resume');
  });
  void audio.setPaused(true);
}
document.addEventListener('visibilitychange', () => { if (document.hidden) pause(); });
window.addEventListener('blur', pause);
window.addEventListener('pagehide', pause);
sound.addEventListener('click', () => { void audio.setEnabled(!audio.enabled); });
resume.addEventListener('click', async () => {
  if (broken || ended) { location.reload(); return; }
  if (!game || !startupReady || starting) return;
  const ticket = ++activation;
  starting = true;
  resume.disabled = true;
  try {
    // start() unlocks the already prepared context directly in this gesture.
    if (!hasStarted) await audio.start();
    else await audio.setPaused(false);
    if (ticket !== activation || broken || document.hidden) return;
    hasStarted = true;
    running = true;
    last = null;
    overlay.hidden = true;
  } catch (error) {
    if (ticket === activation) startupFailure(error);
  } finally {
    if (ticket === activation) {
      starting = false;
      resume.disabled = false;
    }
  }
});
// iOS can interrupt audio independently of the game's pause state. Retry on the
// next gesture, including touch controls whose pointerdown cancels mouse clicks.
for (const type of ['touchend', 'click', 'keydown']) {
  window.addEventListener(type, () => {
    if (running) void audio.resume().catch(error => audio.fail(error));
  }, { passive: true });
}

function tick(now) {
  if (running) attempt(() => {
    if (last !== null) elapsed += Math.min(Math.max(now - last, 0), 100);
    last = now;
    game.advance(elapsed);
    draw();
    audio.send(JSON.parse(game.drain_audio()));
    readWarnings();
    if (game.has_quit()) {
      ended = true;
      controls.clear();
      game.clear_input();
      running = false;
      void audio.setPaused(true);
      showOverlay('Game saved', '', 'Play again');
    }
  });
  requestAnimationFrame(tick);
}

try {
  game = await prepareAssets({
    loadGame: async () => {
      const { default: init, BrowserGame } = await import('./pkg/zelda_mobile.js');
      await init();
      return new BrowserGame();
    },
    audio,
    status: loading,
  });
  controls = bindControls(deck, {
    active: () => running,
    input: (source, action) => attempt(() => game.input(source, action)),
    back: () => attempt(() => game.back()),
  });
  startupReady = true;
  enableControls(true);
  draw();
  readWarnings();
  showOverlay('Zelda Mobile', '', 'Play');
  requestAnimationFrame(tick);
} catch (error) { startupFailure(error); }
