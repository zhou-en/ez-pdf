import Link from 'next/link';

import { auth, signOut } from '@/auth';
import { NavActions } from './NavActions';

/** primary-nav — 56px, canvas background, 1px hairline bottom rule. */
export async function Nav() {
  const session = await auth().catch(() => null);

  return (
    <header className="flex h-14 items-center justify-between border-b border-hairline px-4 sm:px-6">
      <Link href={session?.user ? '/app' : '/'} className="flex min-h-11 items-center font-medium tracking-tight">
        ezpdf
      </Link>

      <nav className="flex items-center gap-2 text-sm">
        <NavActions
          signedIn={Boolean(session?.user)}
          signOutAction={async () => {
            'use server';
            await signOut({ redirectTo: '/' });
          }}
        />
      </nav>
    </header>
  );
}
