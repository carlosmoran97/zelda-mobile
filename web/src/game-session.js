import { bindControls } from '../input.js';
import { AudioOutput } from '../audio.js';

export const initialGameState = {
  status: 'loading',
  title: 'Zelda Mobile',
  message: 'Loading your adventure…',
  button: null,
  commands: [],
  sound: { enabled: false, ready: false },
};

// The session owns browser resources; React owns the interface and its state.
export function createGameSession({ canvas, deck, loadGame, assetBase, onChange, onNotice }) {
  let game, controls, animation;
  let disposed = false;
  let running = false;
  let ended = false;
  let broken = false;
  let elapsed = 0;
  let last = null;
  let lastCommands = '';
  const context = canvas.getContext('2d', { alpha: false });
  const frame = new ImageData(240, 320);
  const events = new AbortController();
  const audio = new AudioOutput(() => {
    if (!disposed) onChange({ sound: { enabled: audio.enabled, ready: audio.ready } });
  }, message => { if (!disposed) onNotice(message); }, assetBase);

  function showOverlay(status, title, message, button) {
    onChange({ status, title, message, button });
  }
  function fatal(error) {
    if (disposed) return;
    broken = true;
    running = false;
    console.error(error);
    void audio.setPaused(true);
    showOverlay('error', 'Adventure interrupted', String(error.message || error), 'Reload');
  }
  function attempt(callback) {
    try { return callback(); } catch (error) { fatal(error); }
  }
  function readWarnings() {
    const warnings = JSON.parse(game.warnings());
    if (warnings.length) onNotice(warnings.join(' '));
  }
  function draw() {
    frame.data.set(game.pixels());
    context.putImageData(frame, 0, 0);
    const commands = game.commands();
    if (commands !== lastCommands) {
      lastCommands = commands;
      onChange({ commands: JSON.parse(commands) });
    }
  }
  function checkpoint() {
    controls?.clear();
    game.clear_input();
    game.checkpoint();
    readWarnings();
  }
  function pause() {
    if (disposed || !game || !running) return;
    running = false;
    last = null;
    attempt(() => {
      checkpoint();
      if (!ended && !broken) showOverlay('paused', 'Game paused', '', 'Resume');
    });
    void audio.setPaused(true);
  }
  function tick(now) {
    if (disposed) return;
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
        showOverlay('ended', 'Game saved', '', 'Play again');
      }
    });
    animation = requestAnimationFrame(tick);
  }

  document.addEventListener('visibilitychange', () => { if (document.hidden) pause(); }, { signal: events.signal });
  for (const type of ['blur', 'pagehide']) window.addEventListener(type, pause, { signal: events.signal });
  // Retry interrupted iOS audio during a gesture, including cancelled touch clicks.
  for (const type of ['touchend', 'click', 'keydown']) {
    window.addEventListener(type, () => { void audio.resume().catch(error => audio.fail(error)); }, { passive: true, signal: events.signal });
  }

  const ready = (async () => {
    try {
      if (!context) throw new Error('This browser does not support the game display.');
      const loaded = await loadGame();
      // StrictMode, navigation, or hot reload can unmount while Wasm loads.
      if (disposed) { loaded.free(); return; }
      game = loaded;
      controls = bindControls(deck, {
        active: () => running,
        input: (source, action) => attempt(() => game.input(source, action)),
        back: () => attempt(() => game.back()),
      });
      draw();
      readWarnings();
      showOverlay('ready', 'Zelda Mobile', '', 'Play');
      animation = requestAnimationFrame(tick);
    } catch (error) { fatal(error); }
  })();

  return {
    ready,
    play() {
      if (disposed) return;
      if (broken || ended) { window.location.reload(); return; }
      if (!game) return;
      running = true;
      last = null;
      onChange({ status: 'playing' });
      void audio.setPaused(false);
      if (!audio.loading && !audio.ready) void audio.setEnabled(true);
    },
    toggleSound() { if (!disposed) void audio.setEnabled(!audio.enabled); },
    dispose() {
      if (disposed) return;
      // Save before releasing the Wasm instance, even when navigating to credits.
      if (game && !broken) attempt(checkpoint);
      disposed = true;
      running = false;
      cancelAnimationFrame(animation);
      events.abort();
      controls?.dispose();
      game?.free();
      void audio.dispose();
    },
  };
}
