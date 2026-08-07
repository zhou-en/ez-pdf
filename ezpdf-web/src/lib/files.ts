'use server';

import { and, desc, eq } from 'drizzle-orm';
import { del } from '@vercel/blob';
import { revalidatePath } from 'next/cache';

import { auth } from '@/auth';
import { getDb } from '@/db';
import { files } from '@/db/schema';

export async function listFiles() {
  const session = await auth();
  const userId = session?.user?.id;
  if (!userId) return [];

  return getDb()
    .select()
    .from(files)
    .where(eq(files.userId, userId))
    .orderBy(desc(files.createdAt));
}

export async function recordSave(input: {
  operation: string;
  sourceName: string;
  sourceBlobUrl: string;
  resultName: string;
  resultBlobUrl: string;
  bytes: number;
}) {
  const session = await auth();
  const userId = session?.user?.id;
  if (!userId) throw new Error('Sign in to save files.');

  await getDb().insert(files).values({ ...input, userId });
  revalidatePath('/library');
}

/**
 * Deletes a saved conversion.
 *
 * Ownership is re-checked here rather than trusted from the caller, and the
 * blobs go before the row: a dangling row is recoverable, but an orphaned
 * private blob is invisible and bills forever.
 */
export async function deleteFile(id: string) {
  const session = await auth();
  const userId = session?.user?.id;
  if (!userId) throw new Error('Unauthorized');

  const db = getDb();
  const [row] = await db
    .select()
    .from(files)
    .where(and(eq(files.id, id), eq(files.userId, userId)))
    .limit(1);

  if (!row) throw new Error('Not found');

  await del([row.sourceBlobUrl, row.resultBlobUrl]);
  await db.delete(files).where(and(eq(files.id, id), eq(files.userId, userId)));

  revalidatePath('/library');
}
