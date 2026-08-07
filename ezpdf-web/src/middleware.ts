import NextAuth from 'next-auth';

import { authConfig } from './auth.config';

// The `authorized` callback in auth.config.ts decides what is public.
export const { auth: middleware } = NextAuth(authConfig);

export default middleware;

export const config = {
  // Everything except Next internals, the auth endpoints and static assets.
  matcher: ['/((?!api/auth|_next/static|_next/image|favicon.ico|.*\\.(?:png|svg|ico|webmanifest|wasm)$).*)'],
};
