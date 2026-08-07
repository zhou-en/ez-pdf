import { describe, expect, it } from 'vitest';

import { credentialsSchema, signUpSchema } from './validation';

describe('credentials validation', () => {
  it('accepts a well-formed email and an 8+ character password', () => {
    expect(credentialsSchema.safeParse({ email: 'a@b.com', password: 'hunter22' }).success).toBe(
      true,
    );
  });

  it('rejects a malformed email', () => {
    const r = credentialsSchema.safeParse({ email: 'nope', password: 'hunter22' });
    expect(r.success).toBe(false);
    if (!r.success) expect(r.error.issues[0].message).toMatch(/valid email/i);
  });

  it('rejects a short password', () => {
    const r = credentialsSchema.safeParse({ email: 'a@b.com', password: 'short' });
    expect(r.success).toBe(false);
    if (!r.success) expect(r.error.issues[0].message).toMatch(/8 characters/i);
  });

  it('sign-up additionally requires a name', () => {
    expect(signUpSchema.safeParse({ email: 'a@b.com', password: 'hunter22' }).success).toBe(false);
    expect(
      signUpSchema.safeParse({ name: 'Ada', email: 'a@b.com', password: 'hunter22' }).success,
    ).toBe(true);
  });

  it('rejects a whitespace-only name', () => {
    expect(
      signUpSchema.safeParse({ name: '   ', email: 'a@b.com', password: 'hunter22' }).success,
    ).toBe(false);
  });
})
