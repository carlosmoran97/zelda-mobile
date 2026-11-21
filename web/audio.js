export class AudioOutput {
  constructor(update, warn) {
    this.update = update;
    this.warn = warn;
    this.enabled = false;
    this.ready = false;
    this.paused = true;
    this.loading = null;
    this.session = null;
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
  async resume() {
    if (!this.context || !this.enabled || this.paused) return;
    // Safari otherwise treats Web Audio as ambient sound and obeys the iPhone's
    // silent switch, even after the player explicitly enables sound.
    try {
      if (globalThis.navigator?.audioSession) globalThis.navigator.audioSession.type = 'playback';
    } catch { /* Audio sessions are optional; keep ordinary Web Audio working. */ }
    if (this.context.state === 'running') return;
    // Start a source during the gesture, before any async work. This also unlocks
    // Safari when the worklet and instrument bank have not finished loading yet.
    const source = this.context.createBufferSource();
    source.buffer = this.context.createBuffer(1, 1, this.context.sampleRate);
    source.connect(this.context.destination);
    source.onended = () => source.disconnect();
    source.start();
    await this.context.resume();
  }
  preload() {
    if (this.loading) return this.loading;
    const session = {};
    this.session = session;
    this.loading = new Promise((resolve, reject) => {
      session.reject = reject;
      session.timeout = setTimeout(() => reject(new Error('Sound loading timed out.')), 90000);
      this.initialize(session).then(resolve, reject);
    }).then(() => {
      if (this.session !== session) throw new Error('Sound loading canceled.');
      clearTimeout(session.timeout);
      session.reject = null;
      this.ready = true;
      this.update();
    }, async error => {
      if (this.session === session) await this.dispose();
      throw error;
    });
    this.update();
    return this.loading;
  }
  async initialize(session) {
    const Context = globalThis.AudioContext || globalThis.webkitAudioContext;
    if (!Context || !globalThis.AudioWorkletNode) throw new Error('This browser does not support game audio.');
    const context = session.context = this.context = new Context({ latencyHint: 'interactive' });
    const check = () => {
      if (this.session !== session) throw new Error('Sound loading canceled.');
    };
    const report = error => {
      if (this.session !== session) return;
      if (this.ready) void this.fail(error);
      else session.reject(error);
    };
    // Asset preparation must finish even when autoplay keeps the context suspended.
    if ((this.paused || !this.enabled) && context.state !== 'suspended') await context.suspend();
    check();
    await context.audioWorklet.addModule(new URL('./audio-worklet.js', import.meta.url));
    check();
    const node = session.node = this.node = new AudioWorkletNode(context, 'zelda-audio', { numberOfInputs: 0, numberOfOutputs: 1, outputChannelCount: [2] });
    node.onprocessorerror = () => report(new Error('Audio playback stopped unexpectedly.'));
    node.connect(context.destination);
    const worker = session.worker = this.worker = new Worker(new URL('./audio-worker.js', import.meta.url), { type: 'module' });
    const channel = new MessageChannel();
    session.channel = channel;
    await new Promise((resolve, reject) => {
      worker.onmessage = ({ data }) => {
        if (this.session !== session) return;
        if (data.type === 'ready') resolve();
        else if (data.type === 'error') {
          const error = new Error(data.message);
          reject(error);
          report(error);
        }
      };
      worker.onerror = event => {
        const error = new Error(event.message || 'The audio worker failed.');
        reject(error);
        report(error);
      };
      worker.postMessage({ type: 'init', sampleRate: context.sampleRate, port: channel.port1 }, [channel.port1]);
    });
    check();
    // Only the current music survives loading; old effects are never replayed.
    if (this.music) worker.postMessage({ type: 'commands', commands: [this.music] });
    node.port.postMessage({ type: 'connect', port: channel.port2 }, [channel.port2]);
    if ((this.paused || !this.enabled) && context.state !== 'suspended') await context.suspend();
    check();
  }
  async start() {
    if (!this.ready) throw new Error('Game audio is not ready.');
    this.enabled = true;
    this.paused = false;
    this.update();
    this.node.port.postMessage({ type: 'flush' });
    // This call must run synchronously in the Play gesture, before the first await.
    await this.resume();
  }
  async setEnabled(enabled) {
    this.enabled = enabled;
    this.update();
    if (enabled) {
      try {
        const loading = this.preload();
        // Unlock within this gesture even when preparation is still pending.
        await Promise.all([loading, this.resume()]);
        if (this.enabled && !this.paused) {
          this.node.port.postMessage({ type: 'flush' });
          await this.resume();
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
        else await this.resume();
      }
    } catch (error) { await this.fail(error); }
  }
  async fail(error) {
    await this.dispose();
    this.warn(`Sound unavailable: ${String(error.message || error)} You can keep playing.`, error);
    this.update();
  }
  async dispose() {
    const session = this.session;
    this.session = null;
    this.ready = false;
    this.enabled = false;
    this.loading = null;
    this.context = null;
    this.worker = null;
    this.node = null;
    this.update();
    if (!session) return;
    clearTimeout(session.timeout);
    session.reject?.(new Error('Sound loading canceled.'));
    if (session.worker) {
      session.worker.onmessage = session.worker.onerror = null;
      session.worker.terminate();
    }
    if (session.node) {
      session.node.onprocessorerror = null;
      session.node.disconnect();
      session.node.port.close();
    }
    session.channel?.port1.close();
    session.channel?.port2.close();
    try { await session.context?.close(); } catch { /* Already closed. */ }
  }
}
