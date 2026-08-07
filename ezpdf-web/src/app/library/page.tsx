import Link from 'next/link';
import { redirect } from 'next/navigation';

import { auth } from '@/auth';
import { DangerZone } from '@/components/DangerZone';
import { FileRow } from '@/components/FileRow';
import { listFiles } from '@/lib/files';
import { formatBytes } from '@/lib/format';

const QUOTA_BYTES = 100 * 1024 * 1024;

export default async function Library() {
  const session = await auth();
  if (!session?.user?.id) redirect('/');

  const rows = await listFiles();
  const used = rows.reduce((sum, r) => sum + r.bytes, 0);

  return (
    <main className="mx-auto w-full max-w-3xl px-4 pb-24 pt-12 sm:px-6">
      <div className="mb-6 flex flex-wrap items-baseline justify-between gap-2">
        <h1 className="text-2xl font-semibold tracking-tight">Your files</h1>
        <span className="text-sm text-mute">
          {formatBytes(used)} of {formatBytes(QUOTA_BYTES)}
        </span>
      </div>

      {rows.length === 0 ? (
        <div className="rounded-lg border border-hairline bg-surface p-6 text-center">
          <p className="text-body">Nothing saved yet.</p>
          <Link href="/app" className="mt-2 inline-block text-sm text-accent-blue hover:underline">
            Convert something →
          </Link>
        </div>
      ) : (
        <ul className="flex flex-col gap-2">
          {rows.map((row) => (
            <FileRow
              key={row.id}
              id={row.id}
              operation={row.operation}
              sourceName={row.sourceName}
              resultName={row.resultName}
              bytes={row.bytes}
              createdAt={row.createdAt.toISOString()}
            />
          ))}
        </ul>
      )}

      <DangerZone email={session.user.email ?? 'your account'} />
    </main>
  );
}
