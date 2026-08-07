import { AuthForm } from '@/components/AuthForm';
import { signIn } from '@/auth';
import { signUpWithPassword } from '@/lib/accounts';

export const metadata = { title: 'Create your ezpdf account' };

export default function SignUp() {
  return (
    <main className="flex min-h-[calc(100dvh-3.5rem)] items-center justify-center px-4 py-10">
      <AuthForm
        mode="signup"
        action={signUpWithPassword}
        googleAction={async () => {
          'use server';
          await signIn('google', { redirectTo: '/app' });
        }}
      />
    </main>
  );
}
