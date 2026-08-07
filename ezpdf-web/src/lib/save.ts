import { upload } from '@vercel/blob/client';

import { recordSave } from './files';
import type { OpResult } from './operations';

/**
 * Uploads the source and result straight from the browser to Blob, then records
 * the pair. Client upload keeps large PDFs out of the function request body.
 *
 * `userId` comes from the server-rendered session and must match what
 * /api/blob/upload derives from the cookie — the route rejects any pathname
 * outside `u/{userId}/`, so a tampered value fails there rather than here.
 */
export async function saveToLibrary(input: {
  op: string;
  userId: string;
  source: { name: string; bytes: Uint8Array };
  result: OpResult;
}) {
  const { op, userId, source, result } = input;

  const resultBlob =
    typeof result.data === 'string'
      ? new Blob([result.data], { type: 'text/markdown' })
      : new Blob([result.data as BlobPart], { type: 'application/pdf' });
  const sourceBlob = new Blob([source.bytes as BlobPart], { type: 'application/pdf' });

  const key = crypto.randomUUID();
  const [uploadedSource, uploadedResult] = await Promise.all([
    upload(`u/${userId}/${key}/source/${source.name}`, sourceBlob, {
      access: 'private',
      handleUploadUrl: '/api/blob/upload',
    }),
    upload(`u/${userId}/${key}/result/${result.filename}`, resultBlob, {
      access: 'private',
      handleUploadUrl: '/api/blob/upload',
    }),
  ]);

  await recordSave({
    operation: op,
    sourceName: source.name,
    sourceBlobUrl: uploadedSource.url,
    resultName: result.filename,
    resultBlobUrl: uploadedResult.url,
    bytes: sourceBlob.size + resultBlob.size,
  });
}
