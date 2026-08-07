import { get } from '@vercel/blob';
import { and, eq } from 'drizzle-orm';

import { auth } from '@/auth';
import { getDb } from '@/db';
import { files } from '@/db/schema';

/**
 * Ownership-checked download.
 *
 * The blob URL is never taken from the client — it is looked up by id scoped to
 * the session user, so knowing someone else's file id yields a 404. Blobs are
 * private, so they are read back through the SDK with the store token rather
 * than by fetching a public URL.
 */
export async function GET(
  request: Request,
  { params }: { params: Promise<{ id: string }> },
) {
  const session = await auth();
  const userId = session?.user?.id;
  if (!userId) return new Response('Unauthorized', { status: 401 });

  const { id } = await params;
  const which =
    new URL(request.url).searchParams.get('which') === 'source' ? 'source' : 'result';

  const [row] = await getDb()
    .select()
    .from(files)
    .where(and(eq(files.id, id), eq(files.userId, userId)))
    .limit(1);

  if (!row) return new Response('Not found', { status: 404 });

  const blobUrl = which === 'source' ? row.sourceBlobUrl : row.resultBlobUrl;
  const filename = which === 'source' ? row.sourceName : row.resultName;

  const blob = await get(blobUrl, { access: 'private' });
  if (!blob || blob.statusCode !== 200) {
    return new Response('Not found', { status: 404 });
  }

  return new Response(blob.stream, {
    headers: {
      'Content-Type': blob.blob.contentType,
      'Content-Disposition': `attachment; filename="${filename.replace(/"/g, '')}"`,
    },
  });
}
