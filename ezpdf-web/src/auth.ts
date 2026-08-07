import { DrizzleAdapter } from '@auth/drizzle-adapter';
import { compare } from 'bcryptjs';
import { eq } from 'drizzle-orm';
import NextAuth from 'next-auth';
import Credentials from 'next-auth/providers/credentials';

import { authConfig } from '@/auth.config';
import { getDb } from '@/db';
import { accounts, users } from '@/db/schema';
import { credentialsSchema } from '@/lib/validation';

export const { handlers, auth, signIn, signOut } = NextAuth(() => ({
  ...authConfig,
  adapter: DrizzleAdapter(getDb(), { usersTable: users, accountsTable: accounts }),
  providers: [
    ...authConfig.providers,
    Credentials({
      credentials: { email: {}, password: {} },
      async authorize(raw) {
        const parsed = credentialsSchema.safeParse(raw);
        if (!parsed.success) return null;

        const { email, password } = parsed.data;
        const [user] = await getDb()
          .select()
          .from(users)
          .where(eq(users.email, email))
          .limit(1);

        // Compare even when the user is missing or is Google-only, so a wrong
        // address and a wrong password take the same time to answer.
        const hash = user?.passwordHash ?? '$2a$10$invalidinvalidinvalidinvalidinvalidinvalidinvalidinvaliduO';
        const ok = await compare(password, hash);
        if (!ok || !user?.passwordHash) return null;

        return { id: user.id, email: user.email, name: user.name, image: user.image };
      },
    }),
  ],
}));
