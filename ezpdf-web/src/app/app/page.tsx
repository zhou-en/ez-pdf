import { redirect } from 'next/navigation';

import { auth } from '@/auth';
import { Converter } from '@/components/Converter';
import { Chip } from '@/components/ui';

export const metadata = { title: 'ezpdf — convert' };

export default async function App() {
  // Middleware already gates this route; the check here is defence in depth and
  // gives us the id the converter needs for uploads.
  const session = await auth();
  if (!session?.user?.id) redirect('/signin');

  return (
    <main className="mx-auto w-full max-w-3xl px-4 pb-20 pt-8 sm:px-6">
      <Converter userId={session.user.id} />
      <Chip tone="info" className="mt-6">
        <span aria-hidden>🔒</span>
        <span>
          Processed entirely in your browser. Nothing is uploaded unless you choose to save it.
        </span>
      </Chip>
    </main>
  );
}
