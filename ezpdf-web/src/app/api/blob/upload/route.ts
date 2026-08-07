import { handleUpload, type HandleUploadBody } from '@vercel/blob/client';

import { auth } from '@/auth';

/**
 * Issues client-upload tokens.
 *
 * Two things make this safe, and both are load-bearing:
 *   1. the session is checked *before* a token is minted, and
 *   2. the pathname is pinned to `u/{userId}/`, so a token cannot be used to
 *      write into another user's prefix.
 * Without either, this is a write-anywhere hole.
 */
export async function POST(request: Request): Promise<Response> {
  const session = await auth();
  const userId = session?.user?.id;
  if (!userId) return Response.json({ error: 'Unauthorized' }, { status: 401 });

  const body = (await request.json()) as HandleUploadBody;

  try {
    const result = await handleUpload({
      body,
      request,
      onBeforeGenerateToken: async (pathname) => {
        if (!pathname.startsWith(`u/${userId}/`)) {
          throw new Error('pathname outside the caller’s prefix');
        }
        return {
          // Private: a public blob URL is a permanent unauthenticated handle to
          // someone's document.
          addRandomSuffix: true,
          allowedContentTypes: ['application/pdf', 'text/markdown', 'text/plain'],
          maximumSizeInBytes: 50 * 1024 * 1024,
          tokenPayload: JSON.stringify({ userId }),
        };
      },
      onUploadCompleted: async () => {
        // The row is written by the client after both uploads succeed, so that
        // a half-finished save never leaves a partial record.
      },
    });
    return Response.json(result);
  } catch (error) {
    return Response.json({ error: (error as Error).message }, { status: 400 });
  }
}
