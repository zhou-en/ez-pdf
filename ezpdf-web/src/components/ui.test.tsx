import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';

import { Button, Card, Chip, Keycap, PillTab } from './ui';

/**
 * These pin the design-system rules that are easy to erode by accident.
 * See DESIGN.md — dark only, white primary, no shadows, accents only as chips.
 */
describe('design system rules', () => {
  it('the primary button is white with black text — there is no tinted primary', () => {
    render(<Button>Convert</Button>);
    const cls = screen.getByRole('button').className;
    expect(cls).toContain('bg-primary');
    expect(cls).toContain('text-on-primary');
  });

  it('no primitive uses a drop shadow — elevation is the surface ladder', () => {
    const { container } = render(
      <>
        <Button>a</Button>
        <PillTab active>b</PillTab>
        <Chip>c</Chip>
        <Card>d</Card>
        <Keycap>e</Keycap>
      </>,
    );
    expect(container.innerHTML).not.toMatch(/\bshadow-/);
  });

  it('chips carry the soft accent background with the accent as text only', () => {
    render(<Chip tone="danger">boom</Chip>);
    const cls = screen.getByText('boom').className;
    expect(cls).toContain('bg-accent-red-soft');
    expect(cls).toContain('text-accent-red');
  });

  it('touch targets clear the 44px minimum, relaxing only on fine pointers', () => {
    render(<Button>Convert</Button>);
    const cls = screen.getByRole('button').className;
    expect(cls).toContain('min-h-11');
    expect(cls).toContain('pointer-fine:min-h-9');
  });

  it('an active pill tab lifts one surface notch', () => {
    render(<PillTab active>Markdown</PillTab>);
    expect(screen.getByRole('tab').className).toContain('bg-elevated');
  });

  it('pill tabs report selection to assistive tech', () => {
    render(
      <>
        <PillTab active>on</PillTab>
        <PillTab>off</PillTab>
      </>,
    );
    const [on, off] = screen.getAllByRole('tab');
    expect(on).toHaveAttribute('aria-selected', 'true');
    expect(off).toHaveAttribute('aria-selected', 'false');
  });

  it('cards use surface + hairline', () => {
    const { container } = render(<Card>x</Card>);
    const cls = container.firstElementChild!.className;
    expect(cls).toContain('bg-surface');
    expect(cls).toContain('border-hairline');
  });

  it('keycaps render as <kbd>', () => {
    render(<Keycap>⌘O</Keycap>);
    expect(screen.getByText('⌘O').tagName).toBe('KBD');
  });
});
