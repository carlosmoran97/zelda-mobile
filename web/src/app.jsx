import { useSyncExternalStore } from 'react';
import { GameHeader } from '@/components/game-header';
import { GameScreen } from '@/components/game-screen';
import { TouchControls } from '@/components/touch-controls';
import { GameNotice } from '@/components/game-notice';
import { Credits } from '@/components/credits';
import { useGame } from '@/hooks/use-game';

function subscribeToHash(callback) {
  window.addEventListener('hashchange', callback);
  return () => window.removeEventListener('hashchange', callback);
}

function Game() {
  const { canvasRef, deckRef, state, notice, play, toggleSound, dismissNotice } = useGame();
  return (
    <>
      <main className="handheld" aria-label="Zelda Mobile game">
        <GameHeader sound={state.sound} onToggleSound={toggleSound} />
        <GameScreen canvasRef={canvasRef} state={state} onPlay={play} />
        <TouchControls deckRef={deckRef} commands={state.commands} />
      </main>
      <GameNotice message={notice} onDismiss={dismissNotice} />
    </>
  );
}

export default function App() {
  const hash = useSyncExternalStore(subscribeToHash, () => window.location.hash);
  if (hash === '#credits' || hash === '#licenses') return <Credits showLicenses={hash === '#licenses'} />;
  return <Game />;
}
