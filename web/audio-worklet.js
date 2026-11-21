// A bounded PCM queue needs no SharedArrayBuffer or special hosting headers.
class GameAudio extends AudioWorkletProcessor {
  constructor() {
    super();
    this.buffer = new Float32Array(16384);
    this.read = 0;
    this.write = 0;
    this.available = 0;
    this.pending = 0;
    this.generation = 0;
    this.port.onmessage = ({ data }) => {
      if (data.type === 'connect') {
        this.source = data.port;
        this.source.onmessage = ({ data: packet }) => {
          if (packet.generation !== this.generation) return;
          const samples = packet.samples;
          this.pending -= samples.length / 2;
          for (let i = 0; i < samples.length; i++) {
            this.buffer[this.write] = samples[i];
            this.write = (this.write + 1) % this.buffer.length;
          }
          this.available += samples.length / 2;
        };
        this.refill();
      } else if (data.type === 'flush') {
        this.generation++;
        this.read = this.write = this.available = this.pending = 0;
        this.refill();
      }
    };
  }
  refill() {
    if (!this.source) return;
    while (this.available + this.pending < 3072) {
      this.pending += 1024;
      this.source.postMessage({ frames: 1024, generation: this.generation });
    }
  }
  process(_inputs, outputs) {
    const channels = outputs[0];
    if (!channels?.length) return true;
    const count = Math.min(channels[0].length, this.available);
    for (let i = 0; i < count; i++) {
      channels[0][i] = this.buffer[this.read];
      if (channels[1]) channels[1][i] = this.buffer[(this.read + 1) % this.buffer.length];
      this.read = (this.read + 2) % this.buffer.length;
    }
    this.available -= count;
    this.refill();
    return true;
  }
}
registerProcessor('zelda-audio', GameAudio);
