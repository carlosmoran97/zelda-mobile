import { Info, Volume2, VolumeX } from 'lucide-react';
import { Button } from '@/components/ui/button';

export function GameHeader({ sound, onToggleSound }) {
  const label = sound.enabled ? 'Mute sound' : 'Enable sound';
  const loading = sound.enabled && !sound.ready;
  const SoundIcon = sound.enabled ? Volume2 : VolumeX;

  return (
    <header className="topbar">
      <span className="brand"><span className="triforce" aria-hidden="true">▲</span> ZELDA <span>MOBILE</span></span>
      <div className="toolbar">
        <Button asChild variant="ghost" size="icon" className="utility">
          <a href="#credits" aria-label="Credits and licenses" title="Credits and licenses"><Info aria-hidden="true" /></a>
        </Button>
        <Button id="sound" variant="ghost" size="icon" className="utility" aria-label={label} aria-pressed={sound.enabled} aria-busy={loading} title={loading ? 'Loading sound…' : label} onClick={onToggleSound}>
          <SoundIcon aria-hidden="true" />
        </Button>
      </div>
    </header>
  );
}
