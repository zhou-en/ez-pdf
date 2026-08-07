import { AuthForm } from '@/components/AuthForm';
import { signIn } from '@/auth';
import { signInWithPassword } from '@/lib/accounts';

export const metadata = { title: 'Sign in to ezpdf' };

export default function SignIn() {
  return (
    <main className="flex min-h-[calc(100dvh-3.5rem)] items-center justify-center px-4 py-10">
      <AuthForm
        mode="signin"
        action={signInWithPassword}
        googleAction={async () => {
          'use server';
          await signIn('google', { redirectTo: '/app' });
        }}
      />
    </main>
  );
}
