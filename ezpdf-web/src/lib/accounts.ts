'use server';

import { hash } from 'bcryptjs';
import { eq } from 'drizzle-orm';
import { del, list } from '@vercel/blob';
import { redirect } from 'next/navigation';

import { auth, signIn, signOut } from '@/auth';
import { getDb } from '@/db';
import { users } from '@/db/schema';
import { signUpSchema } from './validation';

export type FormState = { error?: string } | undefined;

/** Creates a password account, then signs the new user straight in. */
export async function signUpWithPassword(
  _prev: FormState,
  formData: FormData,
): Promise<FormState> {
  const parsed = signUpSchema.safeParse({
    name: formData.get('name'),
    email: formData.get('email'),
    password: formData.get('password'),
  });
  if (!parsed.success) {
    return { error: parsed.error.issues[0]?.message ?? 'Check the form and try again.' };
  }

  const { name, email, password } = parsed.data;
  const db = getDb();

  const [existing] = await db.select().from(users).where(eq(users.email, email)).limit(1);
  if (existing) {
    // Deliberately vague: confirming which addresses exist is an account-
    // enumeration oracle.
    return { error: 'That email is not available.' };
  }

  await db.insert(users).values({ name, email, passwordHash: await hash(password, 10) });

  await signIn('credentials', { email, password, redirectTo: '/app' });
  return undefined;
}

export async function signInWithPassword(
  _prev: FormState,
  formData: FormData,
): Promise<FormState> {
  const email = String(formData.get('email') ?? '');
  const password = String(formData.get('password') ?? '');

  try {
    await signIn('credentials', { email, password, redirectTo: '/app' });
  } catch (error) {
    // next/navigation signals redirects by throwing; let those through.
    if (error instanceof Error && error.message === 'NEXT_REDIRECT') throw error;
    if ((error as { digest?: string })?.digest?.startsWith('NEXT_REDIRECT')) throw error;
    return { error: 'Those details did not match an account.' };
  }
  return undefined;
}

/**
 * Deletes the account and everything it ever stored.
 *
 * Blobs are purged by prefix rather than from the `files` rows, so anything
 * orphaned by a failed save is swept up too. The database rows go last: the
 * `onDelete: 'cascade'` on files and accounts means removing the user row
 * removes the rest.
 */
export async function deleteAccount() {
  const session = await auth();
  const userId = session?.user?.id;
  if (!userId) throw new Error('Unauthorized');

  // No token means this deployment could never have written a blob, so there is
  // nothing to purge. A genuine Blob failure, by contrast, must abort: deleting
  // the rows first would strand the files with no record that they exist.
  if (process.env.BLOB_READ_WRITE_TOKEN) {
    const prefix = `u/${userId}/`;
    let cursor: string | undefined;
    do {
      const page = await list({ prefix, cursor, limit: 1000 });
      if (page.blobs.length > 0) {
        await del(page.blobs.map((b) => b.url));
      }
      cursor = page.hasMore ? page.cursor : undefined;
    } while (cursor);
  }

  // `onDelete: 'cascade'` on accounts and files removes the rest.
  await getDb().delete(users).where(eq(users.id, userId));

  await signOut({ redirect: false });
  redirect('/');
}
