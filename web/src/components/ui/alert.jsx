import { cn } from '@/lib/utils';

export function Alert({ className, ...props }) {
  return <div role="alert" className={cn('relative w-full rounded-lg border p-4 text-sm', className)} {...props} />;
}

export function AlertDescription({ className, ...props }) {
  return <div className={cn('text-sm leading-relaxed', className)} {...props} />;
}
