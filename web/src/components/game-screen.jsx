import { Button } from '@/components/ui/button';

export function GameScreen({ canvasRef, state, onPlay }) {
  return (
    <section className="stage" aria-label="Game screen">
      <div className="screen">
        <canvas ref={canvasRef} id="game" width="240" height="320" aria-label="Zelda Mobile game display" />
        {state.status !== 'playing' && (
          <div id="overlay" className="overlay">
            <span className="crest" aria-hidden="true">▲</span>
            <h1>{state.title}</h1>
            <p role="status">{state.message}</p>
            {state.button && <Button id="resume" className="start" size="lg" onClick={onPlay}>{state.button}</Button>}
          </div>
        )}
      </div>
    </section>
  );
}
