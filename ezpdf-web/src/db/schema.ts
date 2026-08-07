import {
  index,
  integer,
  pgTable,
  primaryKey,
  text,
  timestamp,
  uuid,
} from 'drizzle-orm/pg-core';
import type { AdapterAccountType } from 'next-auth/adapters';

/**
 * Auth.js core tables. `sessions` and `verificationTokens` are deliberately
 * absent: we use the JWT session strategy, so the session lives in a signed
 * cookie and never costs a database read.
 */
export const users = pgTable('users', {
  id: text('id')
    .primaryKey()
    .$defaultFn(() => crypto.randomUUID()),
  name: text('name'),
  email: text('email').unique(),
  emailVerified: timestamp('emailVerified', { mode: 'date' }),
  image: text('image'),
  // Null for Google-only accounts. Present when the user signed up with a
  // password, which also makes the app testable without an OAuth round-trip.
  passwordHash: text('passwordHash'),
});

export const accounts = pgTable(
  'accounts',
  {
    userId: text('userId')
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    type: text('type').$type<AdapterAccountType>().notNull(),
    provider: text('provider').notNull(),
    providerAccountId: text('providerAccountId').notNull(),
    refresh_token: text('refresh_token'),
    access_token: text('access_token'),
    expires_at: integer('expires_at'),
    token_type: text('token_type'),
    scope: text('scope'),
    id_token: text('id_token'),
    session_state: text('session_state'),
  },
  (account) => [
    primaryKey({ columns: [account.provider, account.providerAccountId] }),
  ],
);

/**
 * One row per saved conversion, holding both the source and the result so
 * "delete my files" removes the pair.
 */
export const files = pgTable(
  'files',
  {
    id: uuid('id').primaryKey().defaultRandom(),
    userId: text('userId')
      .notNull()
      .references(() => users.id, { onDelete: 'cascade' }),
    operation: text('operation').notNull(),
    sourceName: text('sourceName').notNull(),
    sourceBlobUrl: text('sourceBlobUrl').notNull(),
    resultName: text('resultName').notNull(),
    resultBlobUrl: text('resultBlobUrl').notNull(),
    bytes: integer('bytes').notNull(),
    createdAt: timestamp('createdAt', { withTimezone: true }).notNull().defaultNow(),
  },
  // The library's only query pattern.
  (t) => [index('files_user_created_idx').on(t.userId, t.createdAt.desc())],
);
