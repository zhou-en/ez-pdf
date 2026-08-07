import { PageGridDemo } from '@/components/PageGridDemo';
import { Button, Card, Chip, Keycap, PillTab } from '@/components/ui';

/**
 * Every primitive rendered once. The cheapest way to keep the design system
 * honest — a drop shadow or a stray light surface shows up here immediately.
 */
export default function Styleguide() {
  return (
    <main className="mx-auto flex w-full max-w-3xl flex-col gap-10 px-4 py-12 sm:px-6">
      <h1 className="text-3xl font-semibold tracking-tight">Styleguide</h1>

      <Section title="Surface ladder — elevation without shadows">
        <div className="flex flex-wrap gap-3">
          {[
            ['canvas', 'bg-canvas'],
            ['surface', 'bg-surface'],
            ['elevated', 'bg-elevated'],
            ['card', 'bg-card'],
          ].map(([name, cls]) => (
            <div
              key={name}
              className={`flex size-24 items-center justify-center rounded-lg border border-hairline text-xs text-mute ${cls}`}
            >
              {name}
            </div>
          ))}
        </div>
      </Section>

      <Section title="Buttons — white is the only primary">
        <div className="flex flex-wrap items-center gap-3">
          <Button>Primary</Button>
          <Button variant="secondary">Secondary</Button>
          <Button variant="tertiary">Tertiary</Button>
          <Button disabled>Disabled</Button>
        </div>
      </Section>

      <Section title="Pill tabs">
        <div className="flex gap-1">
          <PillTab active>Markdown</PillTab>
          <PillTab>Merge</PillTab>
          <PillTab>Split</PillTab>
        </div>
      </Section>

      <Section title="Chips — the only sanctioned use of a saturated accent">
        <div className="flex flex-wrap gap-2">
          <Chip tone="info">Processed in your browser</Chip>
          <Chip tone="success">Saved to your library</Chip>
          <Chip tone="danger">No extractable text layer — try ocrmypdf</Chip>
          <Chip tone="warning">Large file</Chip>
        </div>
      </Section>

      <Section title="Keycaps">
        <div className="flex items-center gap-3 text-sm text-mute">
          <Keycap>⌘O</Keycap> open · <Keycap>⏎</Keycap> convert
        </div>
      </Section>

      <Section title="Page tiles — true aspect, from real page dimensions">
        <PageGridDemo />
      </Section>

      <Section title="Card">
        <Card className="p-4">
          <p className="text-sm text-body">
            surface + hairline, padding stays at 16–24px. Never 32+.
          </p>
        </Card>
      </Section>

      <Section title="Type scale">
        <div className="flex flex-col gap-2">
          <p className="text-5xl font-semibold tracking-tight">Display</p>
          <p className="text-2xl font-medium">Heading</p>
          <p className="text-base text-body">Body — the quick brown fox jumps over the lazy dog.</p>
          <p className="text-sm text-mute">Small / muted</p>
          <p className="font-mono text-xs text-body">mono 12px — ## Quarterly Report</p>
        </div>
      </Section>
    </main>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="flex flex-col gap-3">
      <h2 className="text-xs uppercase tracking-wider text-ash">{title}</h2>
      {children}
    </section>
  );
}
