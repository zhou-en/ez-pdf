'use client';

import Link from 'next/link';
import { useActionState } from 'react';

import { Button, Card, Chip } from './ui';
import type { FormState } from '@/lib/accounts';

const FIELD =
  'h-11 w-full rounded-md border border-hairline bg-elevated px-3 text-base text-ink placeholder:text-ash';

/** Shared shell for /signin and /signup — same chrome, different fields. */
export function AuthForm({
  mode,
  action,
  googleAction,
}: {
  mode: 'signin' | 'signup';
  action: (prev: FormState, data: FormData) => Promise<FormState>;
  googleAction: () => Promise<void>;
}) {
  const [state, formAction, pending] = useActionState(action, undefined);
  const signUp = mode === 'signup';

  return (
    <Card className="w-full max-w-sm p-6">
      <h1 className="text-xl font-semibold tracking-tight">
        {signUp ? 'Create your account' : 'Welcome back'}
      </h1>
      <p className="mt-1 text-sm text-mute">
        {signUp ? 'Free, and your files stay in your browser.' : 'Sign in to reach your library.'}
      </p>

      <form action={formAction} className="mt-6 flex flex-col gap-3">
        {signUp && (
          <label className="flex flex-col gap-1.5">
            <span className="text-xs uppercase tracking-wide text-mute">Name</span>
            <input name="name" required autoComplete="name" className={FIELD} placeholder="Ada Lovelace" />
          </label>
        )}
        <label className="flex flex-col gap-1.5">
          <span className="text-xs uppercase tracking-wide text-mute">Email</span>
          <input
            name="email"
            type="email"
            required
            autoComplete="email"
            className={FIELD}
            placeholder="you@example.com"
          />
        </label>
        <label className="flex flex-col gap-1.5">
          <span className="text-xs uppercase tracking-wide text-mute">Password</span>
          <input
            name="password"
            type="password"
            required
            minLength={8}
            autoComplete={signUp ? 'new-password' : 'current-password'}
            className={FIELD}
            placeholder={signUp ? 'At least 8 characters' : '••••••••'}
          />
        </label>

        <div aria-live="polite" className="empty:hidden">
          {state?.error && <Chip tone="danger">{state.error}</Chip>}
        </div>

        <Button type="submit" disabled={pending} className="h-11 w-full">
          {pending ? 'Just a moment…' : signUp ? 'Create account' : 'Sign in'}
        </Button>
      </form>

      <div className="my-5 flex items-center gap-3 text-xs text-ash">
        <span className="h-px flex-1 bg-hairline" />
        or
        <span className="h-px flex-1 bg-hairline" />
      </div>

      <form action={googleAction}>
        <Button type="submit" variant="tertiary" className="h-11 w-full">
          Continue with Google
        </Button>
      </form>

      <p className="mt-6 text-center text-sm text-mute">
        {signUp ? 'Already have an account? ' : 'New here? '}
        <Link
          href={signUp ? '/signin' : '/signup'}
          className="text-accent-blue underline-offset-2 hover:underline"
        >
          {signUp ? 'Sign in' : 'Create one'}
        </Link>
      </p>
    </Card>
  );
}
