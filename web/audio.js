export class AudioOutput {
  constructor(update, warn) {
    this.update = update;
    this.warn = warn;
    this.enabled = false;
    this.ready = false;
    this.paused = true;
    this.music = null;
    this.effect = null;
  }
  send(commands) {
    for (const command of commands) {
      if (command.type === 'play' && command.data.name.endsWith('.mid')) this.music = structuredClone(command);
      if (command.type === 'play' && command.data.name.endsWith('.wav')) this.effect = command.data.id;
      if (command.type === 'stop' && command.data === this.music?.data.id) this.music = null;
      if (command.type === 'volume' && command.data[0] === this.music?.data.id) this.music.data.volume = command.data[1];
    }
    if (this.ready && commands.length) {
      this.worker.postMessage({ type: 'commands', commands: this.enabled ? commands : commands.filter(c => c.type !== 'play' || c.data.name.endsWith('.mid')) });
    }
  }
  async initialize() {
    const Context = globalThis.AudioContext || globalThis.webkitAudioContext;
    if (!Context || !globalThis.AudioWorkletNode) throw new Error('This browser does not support game audio.');
    this.context = new Context({ latencyHint: 'interactive' });
    // Resume immediately within the user gesture, before fetching any assets.
    if (!this.paused) await this.context.resume();
    await this.context.audioWorklet.addModule(new URL('./audio-worklet.js', import.meta.url));
    this.node = new AudioWorkletNode(this.context, 'zelda-audio', { numberOfInputs: 0, numberOfOutputs: 1, outputChannelCount: [2] });
    this.node.onprocessorerror = () => { void this.fail(new Error('Audio playback stopped unexpectedly. Tap Sound to retry.')); };
    this.node.connect(this.context.destination);
    this.worker = new Worker(new URL('./audio-worker.js', import.meta.url), { type: 'module' });
    const channel = new MessageChannel();
    await new Promise((resolve, reject) => {
      const timeout = setTimeout(() => reject(new Error('Sound loading timed out. Tap Sound to retry.')), 90000);
      this.worker.onmessage = ({ data }) => {
        if (data.type === 'ready') { clearTimeout(timeout); resolve(); }
        else if (data.type === 'error') {
          clearTimeout(timeout);
          if (!this.ready) reject(new Error(data.message)); else this.fail(new Error(data.message));
        }
      };
      this.worker.onerror = event => {
        clearTimeout(timeout);
        const error = new Error(event.message || 'The audio worker failed.');
        if (this.ready) void this.fail(error); else reject(error);
      };
      this.worker.postMessage({ type: 'init', sampleRate: this.context.sampleRate, port: channel.port1 }, [channel.port1]);
    });
    this.ready = true;
    // Only the current music survives loading; old effects are never replayed.
    if (this.music) this.worker.postMessage({ type: 'commands', commands: [this.music] });
    this.node.port.postMessage({ type: 'connect', port: channel.port2 }, [channel.port2]);
    if (this.paused || !this.enabled) await this.context.suspend();
    this.update();
  }
  async setEnabled(enabled) {
    this.enabled = enabled;
    this.update();
    if (enabled) {
      try {
        if (!this.loading) this.loading = this.initialize();
        // An existing suspended context also needs resuming during this gesture.
        if (this.context && !this.paused) await this.context.resume();
        await this.loading;
        if (this.enabled && !this.paused) {
          this.node.port.postMessage({ type: 'flush' });
          await this.context.resume();
        }
      } catch (error) { await this.fail(error); }
    } else {
      if (this.ready && this.effect !== null) this.worker.postMessage({ type: 'commands', commands: [{ type: 'stop', data: this.effect }] });
      await this.context?.suspend();
    }
    this.update();
  }
  async setPaused(paused) {
    this.paused = paused;
    try {
      if (this.context) {
        if (paused || !this.enabled) await this.context.suspend();
        else await this.context.resume();
      }
    } catch (error) { await this.fail(error); }
  }
  async fail(error) {
    this.ready = false;
    this.enabled = false;
    this.loading = null;
    this.worker?.terminate();
    try { await this.context?.close(); } catch { /* Already closed. */ }
    this.context = null;
    this.warn(`Sound unavailable: ${String(error.message || error)} You can keep playing.`);
    this.update();
  }
}
