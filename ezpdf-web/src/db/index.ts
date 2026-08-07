import { neon } from '@neondatabase/serverless';
import { drizzle as drizzleNeon } from 'drizzle-orm/neon-http';
import { drizzle as drizzlePg } from 'drizzle-orm/node-postgres';
import { Pool } from 'pg';

import * as schema from './schema';

/**
 * Neon in production, plain Postgres locally.
 *
 * @neondatabase/serverless speaks Neon's HTTP protocol and cannot talk to an
 * ordinary Postgres, so a local database would otherwise be untestable. The
 * split is chosen from the connection string: anything that is not a Neon host
 * goes through node-postgres.
 */
function isNeon(url: string): boolean {
  return url.includes('neon.tech') || url.includes('neon.build');
}

function create() {
  const url = process.env.DATABASE_URL;
  if (!url) throw new Error('DATABASE_URL is not set');

  return isNeon(url)
    ? drizzleNeon(neon(url), { schema })
    : drizzlePg(new Pool({ connectionString: url }), { schema });
}

type Db = ReturnType<typeof create>;
let cached: Db | null = null;

/**
 * Lazily-created Drizzle client.
 *
 * Deliberately a plain function and not a `Proxy`-wrapped singleton: Auth.js
 * inspects the adapter object (checking for methods, iterating properties), and
 * a Proxy intercepts those checks and makes the auth request chain hang with no
 * error. Lazy also keeps `next build` working before the database exists, since
 * top-level module code runs at build time.
 */
export function getDb(): Db {
  cached ??= create();
  return cached;
}
