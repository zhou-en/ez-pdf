import { describe, expect, it } from 'vitest';

import { PUBLIC_ROUTES, authConfig } from './auth.config';

const authorized = authConfig.callbacks.authorized!;

function check(pathname: string, signedIn: boolean) {
  return authorized({
    auth: signedIn ? ({ user: { id: 'u1' } } as never) : null,
    request: { nextUrl: { pathname } } as never,
  } as never);
}

/**
 * Route gating is deny-by-default, so these tests are the guard against a new
 * page shipping unprotected.
 */
describe('route protection', () => {
  it('allows the landing and auth pages when signed out', () => {
    for (const route of PUBLIC_ROUTES) expect(check(route, false)).toBe(true);
  });

  it('blocks the app and library when signed out', () => {
    expect(check('/app', false)).toBe(false);
    expect(check('/library', false)).toBe(false);
  });

  it('blocks an unknown route when signed out — new pages are private by default', () => {
    expect(check('/some/future/page', false)).toBe(false);
  });

  it('allows the app and library when signed in', () => {
    expect(check('/app', true)).toBe(true);
    expect(check('/library', true)).toBe(true);
  });

  it('always allows the auth endpoints, so sign-in can complete', () => {
    expect(check('/api/auth/callback/google', false)).toBe(true);
  });

  it('uses JWT sessions, which the Credentials provider requires', () => {
    expect(authConfig.session.strategy).toBe('jwt');
  });
});
