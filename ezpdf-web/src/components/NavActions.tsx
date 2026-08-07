'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';

import { Button } from './ui';

/**
 * The nav's auth controls.
 *
 * Hidden on /signin and /signup: showing a "Sign in" button above a sign-in
 * form is noise, and the nav button previously fired Google OAuth, which made
 * it look like the form's submit.
 */
export function NavActions({
  signedIn,
  signOutAction,
}: {
  signedIn: boolean;
  signOutAction: () => Promise<void>;
}) {
  const pathname = usePathname();
  if (pathname === '/signin' || pathname === '/signup') return null;

  if (!signedIn) {
    return (
      <Link href="/signin">
        <Button>Sign in</Button>
      </Link>
    );
  }

  return (
    <>
      <Link
        href="/library"
        className="flex min-h-11 items-center rounded-md px-3 text-body hover:text-ink"
      >
        Library
      </Link>
      <form action={signOutAction}>
        <Button variant="secondary" type="submit">
          Sign out
        </Button>
      </form>
    </>
  );
}
