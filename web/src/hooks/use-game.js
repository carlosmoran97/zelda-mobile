import { useEffect, useRef, useState } from 'react';
import { createGameSession, initialGameState } from '@/game-session';

let gameModule;
async function loadGame() {
  gameModule ??= (async () => {
    const url = new URL(`${import.meta.env.BASE_URL}pkg/zelda_mobile.js`, document.baseURI);
    const wasm = await import(/* @vite-ignore */ url.href);
    await wasm.default();
    return wasm;
  })();
  const wasm = await gameModule;
  return new wasm.BrowserGame();
}

export function useGame() {
  const canvasRef = useRef(null);
  const deckRef = useRef(null);
  const sessionRef = useRef(null);
  const [state, setState] = useState(initialGameState);
  const [notice, setNotice] = useState(null);

  useEffect(() => {
    const session = createGameSession({
      canvas: canvasRef.current,
      deck: deckRef.current,
      loadGame,
      assetBase: new URL(import.meta.env.BASE_URL, document.baseURI),
      onChange: patch => setState(previous => ({ ...previous, ...patch })),
      onNotice: setNotice,
    });
    sessionRef.current = session;
    return () => {
      session.dispose();
      sessionRef.current = null;
    };
  }, []);

  return {
    canvasRef, deckRef, state, notice,
    play: () => sessionRef.current?.play(),
    toggleSound: () => sessionRef.current?.toggleSound(),
    dismissNotice: () => setNotice(null),
  };
}
