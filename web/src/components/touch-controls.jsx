import { Button } from '@/components/ui/button';

const directions = [
  { action: 0, name: 'up', symbol: '▲' },
  { action: 2, name: 'left', symbol: '◀' },
  { action: 3, name: 'right', symbol: '▶' },
  { action: 1, name: 'down', symbol: '▼' },
];

export function TouchControls({ deckRef, commands }) {
  return (
    <section ref={deckRef} className="control-deck" aria-label="Touch controls">
      <div className="commands">
        {['Left', 'Right'].map((side, index) => (
          <Button key={side} data-action={7 + index} id={`command-${side.toLowerCase()}`} className="command" variant="secondary" disabled={!commands[index]}>
            {commands[index] || `${side} command`}
          </Button>
        ))}
      </div>
      <div className="play-controls">
        <div className="dpad" aria-label="Directional pad">
          {directions.map(({ action, name, symbol }) => (
            <Button key={action} data-action={action} className={`direction ${name}`} variant="secondary" aria-label={`Move ${name}`}>
              <span aria-hidden="true">{symbol}</span>
            </Button>
          ))}
          <span className="pad-center" aria-hidden="true">◆</span>
        </div>
        <div className="action-buttons">
          <Button data-action="5" className="action secondary" variant="secondary" aria-label="B: secondary action and back">B</Button>
          <Button data-action="4" className="action primary" aria-label="A: primary action and confirm">A</Button>
        </div>
      </div>
    </section>
  );
}
