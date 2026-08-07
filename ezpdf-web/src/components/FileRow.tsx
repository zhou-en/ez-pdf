'use client';

import { useState, useTransition } from 'react';

import { deleteFile } from '@/lib/files';
import { Button, Chip } from './ui';
import { formatBytes } from '@/lib/format';

/** store-extension-card layout: tile left, metadata centre, actions right. */
export function FileRow(props: {
  id: string;
  operation: string;
  sourceName: string;
  resultName: string;
  bytes: number;
  createdAt: string;
}) {
  const { id, operation, sourceName, resultName, bytes, createdAt } = props;
  const [confirming, setConfirming] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pending, startTransition] = useTransition();
  const [removed, setRemoved] = useState(false);

  if (removed) return null;

  function remove() {
    setError(null);
    startTransition(async () => {
      try {
        await deleteFile(id);
        setRemoved(true);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Could not delete that file.');
        setConfirming(false);
      }
    });
  }

  return (
    <li className="flex flex-wrap items-center gap-3 rounded-md border border-hairline bg-surface p-4">
      <div
        className="flex size-12 shrink-0 items-center justify-center rounded-md bg-card text-[10px] uppercase tracking-wide text-mute"
        aria-hidden
      >
        {resultName.split('.').pop()}
      </div>

      <div className="min-w-0 flex-1">
        <p className="truncate text-sm">
          <span className="text-mute">{sourceName}</span>
          <span className="mx-1.5 text-ash">→</span>
          <span className="font-medium">{resultName}</span>
        </p>
        <p className="mt-0.5 text-xs text-mute">
          <span className="capitalize">{operation}</span> · {formatBytes(bytes)} ·{' '}
          {new Date(createdAt).toLocaleDateString()}
        </p>
        {error && (
          <Chip tone="danger" className="mt-2">
            {error}
          </Chip>
        )}
      </div>

      <div className="flex shrink-0 items-center gap-2">
        <a href={`/api/files/${id}/download`} download>
          <Button variant="tertiary">Download</Button>
        </a>
        {confirming ? (
          <>
            <Button variant="secondary" onClick={() => setConfirming(false)} disabled={pending}>
              Cancel
            </Button>
            <Button onClick={remove} disabled={pending}>
              {pending ? 'Deleting…' : 'Confirm'}
            </Button>
          </>
        ) : (
          <Button variant="secondary" onClick={() => setConfirming(true)}>
            Delete
          </Button>
        )}
      </div>
    </li>
  );
}
