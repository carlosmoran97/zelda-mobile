import init, { BrowserAudio } from './pkg/zelda_mobile.js';

let synth;
self.onmessage = async ({ data }) => {
  try {
    if (data.type === 'init') {
      const response = await fetch(new URL('./assets/audio/GeneralUser-GS.sf2', import.meta.url));
      if (!response.ok) throw new Error(`Sound download failed (${response.status})`);
      const [font] = await Promise.all([response.arrayBuffer(), init()]);
      synth = new BrowserAudio(new Uint8Array(font), data.sampleRate);
      const port = data.port;
      port.onmessage = ({ data: request }) => {
        try {
          const samples = synth.render(request.frames);
          port.postMessage({ samples, generation: request.generation }, [samples.buffer]);
        } catch (error) { self.postMessage({ type: 'error', message: String(error) }); }
      };
      self.postMessage({ type: 'ready' });
    } else if (data.type === 'commands' && synth) {
      synth.command(JSON.stringify(data.commands));
    }
  } catch (error) {
    self.postMessage({ type: 'error', message: String(error) });
  }
};
