import { X } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';

export function GameNotice({ message, onDismiss }) {
  if (!message) return null;
  return (
    <Alert id="notice">
      <AlertDescription>{message}</AlertDescription>
      <Button id="dismiss" variant="ghost" size="icon" aria-label="Dismiss notice" onClick={onDismiss}><X aria-hidden="true" /></Button>
    </Alert>
  );
}
