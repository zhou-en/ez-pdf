'use client';

import { useState, useTransition } from 'react';

import { deleteAccount } from '@/lib/accounts';
import { Button, Chip } from './ui';

const PHRASE = 'delete my account';

/**
 * Account deletion. Typing the phrase is deliberate friction — this removes
 * every stored file and cannot be undone.
 */
export function DangerZone({ email }: { email: string }) {
  const [open, setOpen] = useState(false);
  const [typed, setTyped] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [pending, start] = useTransition();

  return (
    <section className="mt-12 rounded-lg border border-hairline bg-surface p-5">
      <h2 className="text-sm font-medium">Delete account</h2>
      <p className="mt-1.5 text-sm text-mute">
        Permanently removes <span className="text-body">{email}</span>, every file you have saved,
        and every file the app generated for you. This cannot be undone.
      </p>

      {open ? (
        <div className="mt-4 flex flex-col gap-3">
          <label className="flex flex-col gap-1.5">
            <span className="text-xs text-mute">
              Type <span className="text-body">{PHRASE}</span> to confirm
            </span>
            <input
              value={typed}
              onChange={(e) => setTyped(e.target.value)}
              aria-label={`Type ${PHRASE} to confirm`}
              className="h-11 w-full max-w-xs rounded-md border border-hairline bg-elevated px-3 text-base text-ink"
            />
          </label>

          <div aria-live="polite" className="empty:hidden">
            {error && <Chip tone="danger">{error}</Chip>}
          </div>

          <div className="flex flex-wrap gap-2">
            <Button variant="secondary" className="h-11" onClick={() => setOpen(false)} disabled={pending}>
              Cancel
            </Button>
            <Button
              className="h-11"
              disabled={typed !== PHRASE || pending}
              onClick={() =>
                start(async () => {
                  setError(null);
                  try {
                    await deleteAccount();
                  } catch (err) {
                    if ((err as { digest?: string })?.digest?.startsWith('NEXT_REDIRECT')) throw err;
                    setError(err instanceof Error ? err.message : 'Could not delete the account.');
                  }
                })
              }
            >
              {pending ? 'Deleting everything…' : 'Delete my account'}
            </Button>
          </div>
        </div>
      ) : (
        <Button variant="tertiary" className="mt-4 h-11" onClick={() => setOpen(true)}>
          Delete account…
        </Button>
      )}
    </section>
  );
}
