import Link from 'next/link';

import { auth } from '@/auth';
import { Button, Card, Keycap } from '@/components/ui';

/**
 * Public landing page. This is the only route an anonymous visitor sees, and
 * it doubles as the showcase tile on the dashboard.
 */
export default async function Landing() {
  const session = await auth().catch(() => null);
  const href = session?.user ? '/app' : '/signup';

  return (
    <main className="relative overflow-hidden">
      {/* hero-stripe-band — used exactly once, per the system's one-band rule */}
      <div className="hero-stripes pointer-events-none absolute inset-x-0 top-0 -z-10 h-80" aria-hidden />

      <section className="mx-auto w-full max-w-3xl px-4 pb-16 pt-16 sm:px-6 sm:pt-24">
        <h1 className="text-4xl font-semibold leading-[1.05] tracking-tight sm:text-6xl">
          Every PDF tool,
          <br />
          without the upload.
        </h1>
        <p className="mt-5 max-w-xl text-lg text-body sm:text-xl">
          Merge, split, rotate and convert PDFs to Markdown. Everything runs inside your browser —
          your documents never touch a server unless you ask us to keep them.
        </p>

        <div className="mt-8 flex flex-wrap items-center gap-3">
          <Link href={href}>
            <Button className="h-11 px-6">
              {session?.user ? 'Open ezpdf' : 'Get started — it’s free'}
            </Button>
          </Link>
          {!session?.user && (
            <Link href="/signin">
              <Button variant="tertiary" className="h-11 px-6">
                Sign in
              </Button>
            </Link>
          )}
        </div>
      </section>

      <section className="mx-auto grid w-full max-w-3xl gap-3 px-4 pb-16 sm:grid-cols-3 sm:px-6">
        {[
          {
            title: 'Private by construction',
            body: 'Conversion happens in WebAssembly on your device. There is no upload step to opt out of.',
          },
          {
            title: 'Lossless',
            body: 'Pages are moved at the PDF object level. Nothing is re-rendered or re-compressed.',
          },
          {
            title: 'Instant',
            body: 'No queue, no round-trip. A typical document converts in well under a second.',
          },
        ].map((f) => (
          <Card key={f.title} className="p-5">
            <h2 className="text-sm font-medium">{f.title}</h2>
            <p className="mt-1.5 text-sm leading-relaxed text-mute">{f.body}</p>
          </Card>
        ))}
      </section>

      <section className="mx-auto w-full max-w-3xl px-4 pb-24 sm:px-6">
        <h2 className="text-xs uppercase tracking-wider text-ash">What you can do</h2>
        <ul className="mt-3 grid gap-2 sm:grid-cols-2">
          {[
            ['Markdown', 'Headings, lists and tables extracted as clean Markdown.'],
            ['Merge', 'Combine any number of PDFs, in the order you choose.'],
            ['Split', 'Keep just the pages you want.'],
            ['Remove', 'Drop pages you do not need.'],
            ['Rotate', 'Turn pages 90°, 180° or 270°.'],
            ['Your library', 'Save results to your account — and delete them whenever.'],
          ].map(([title, body]) => (
            <li key={title} className="rounded-md border border-hairline bg-surface p-4">
              <p className="text-sm font-medium">{title}</p>
              <p className="mt-1 text-sm text-mute">{body}</p>
            </li>
          ))}
        </ul>

        <p className="mt-8 flex flex-wrap items-center gap-2 text-sm text-mute">
          Prefer a terminal? <Keycap>brew install ezpdf</Keycap>
          <span>— the CLI and desktop apps need no account at all.</span>
        </p>
      </section>
    </main>
  );
}
