import type { NextAuthConfig } from 'next-auth';
import Google from 'next-auth/providers/google';

/**
 * The edge-safe half of the auth setup.
 *
 * Middleware runs on the edge runtime, where neither the Drizzle/Neon adapter
 * nor bcrypt can load. Keeping the route-protection rules here — and the
 * adapter plus the Credentials provider in `auth.ts` — is what lets middleware
 * gate routes without dragging Node-only code into the edge bundle.
 */
export const PUBLIC_ROUTES = ['/', '/signin', '/signup'];

export const authConfig = {
  providers: [Google],
  pages: { signIn: '/signin' },
  session: { strategy: 'jwt' },
  callbacks: {
    // Every route is private unless listed above. Deny-by-default: a new page
    // is protected the moment it exists, rather than when someone remembers.
    authorized({ auth, request }) {
      const { pathname } = request.nextUrl;
      if (PUBLIC_ROUTES.includes(pathname)) return true;
      if (pathname.startsWith('/api/auth')) return true;
      return Boolean(auth?.user);
    },
    jwt({ token, user }) {
      if (user) token.uid = user.id;
      return token;
    },
    session({ session, token }) {
      if (token.uid) session.user.id = token.uid as string;
      return session;
    },
  },
} satisfies NextAuthConfig;
