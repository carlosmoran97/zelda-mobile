import init, { BrowserGame } from './pkg/zelda_mobile.js';
import { bindControls } from './input.js';
import { AudioOutput } from './audio.js';

const $ = selector => document.querySelector(selector);
const canvas = $('#game');
const context = canvas.getContext('2d', { alpha: false });
const frame = new ImageData(240, 320);
const overlay = $('#overlay');
const resume = $('#resume');
const sound = $('#sound');
let game, controls, running = false, ended = false, broken = false;
let elapsed = 0, last = null, lastCommands = '';

function notice(message) {
  $('#notice-text').textContent = message;
  $('#notice').hidden = false;
}
$('#dismiss').addEventListener('click', () => { $('#notice').hidden = true; });
const audio = new AudioOutput(() => {
  sound.textContent = audio.enabled ? (audio.ready ? 'Sound on' : 'Sound…') : 'Sound off';
  sound.setAttribute('aria-pressed', String(audio.enabled));
  sound.setAttribute('aria-label', audio.enabled ? 'Mute sound' : 'Enable sound');
  $('#status').textContent = audio.enabled && !audio.ready ? 'Loading sound…' : 'Saves stay in this browser';
}, notice);

function showOverlay(title, text, button) {
  $('#overlay-title').textContent = title;
  $('#overlay-text').textContent = text;
  resume.textContent = button;
  resume.hidden = false;
  overlay.hidden = false;
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
  if (!game || !running) return;
  running = false;
  last = null;
  attempt(() => {
    controls.clear();
    game.clear_input();
    game.checkpoint();
    readWarnings();
    if (!ended && !broken) showOverlay('Adventure paused', 'Take your time. Tap Resume to continue.', 'Resume');
  });
  void audio.setPaused(true);
}
document.addEventListener('visibilitychange', () => { if (document.hidden) pause(); });
window.addEventListener('blur', pause);
window.addEventListener('pagehide', pause);
sound.addEventListener('click', () => { void audio.setEnabled(!audio.enabled); });
resume.addEventListener('click', () => {
  if (broken || ended) { location.reload(); return; }
  if (!game) return;
  running = true;
  last = null;
  overlay.hidden = true;
  void audio.setPaused(false);
  if (!audio.loading && !audio.ready) void audio.setEnabled(true);
});

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
      showOverlay('Until next time', 'Play again to return to your save slots.', 'Play again');
    }
  });
  requestAnimationFrame(tick);
}

try {
  await init();
  game = new BrowserGame();
  controls = bindControls($('.control-deck'), {
    active: () => running,
    input: (source, action) => attempt(() => game.input(source, action)),
    back: () => attempt(() => game.back()),
  });
  draw();
  readWarnings();
  showOverlay('Zelda Mobile', 'A familiar adventure, ready for your thumbs.', 'Play');
  $('#status').textContent = 'Saves stay in this browser';
  requestAnimationFrame(tick);
} catch (error) { fatal(error); }
