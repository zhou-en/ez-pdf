/**
 * Primitives from the Raycast design system (see ../../DESIGN.md).
 *
 * Rules these encode, so they are not re-litigated at each call site:
 *   - white is the only primary action colour
 *   - elevation is the surface ladder, never a shadow
 *   - saturated accents appear only as -soft chips, never on chrome or text
 */
import type { ButtonHTMLAttributes, ReactNode } from 'react';

type ButtonVariant = 'primary' | 'secondary' | 'tertiary';

const BUTTON_VARIANTS: Record<ButtonVariant, string> = {
  // button-primary — the universal CTA. White pill, black text.
  primary:
    'bg-primary text-on-primary hover:bg-primary-pressed active:bg-primary-pressed disabled:bg-elevated disabled:text-ash disabled:cursor-not-allowed',
  // button-secondary — transparent text button.
  secondary: 'bg-transparent text-ink hover:bg-elevated disabled:text-ash',
  // button-tertiary — soft surface button.
  tertiary: 'bg-elevated text-ink hover:bg-card disabled:text-ash',
};

export function Button({
  variant = 'primary',
  className = '',
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement> & { variant?: ButtonVariant }) {
  return (
    <button
      className={`inline-flex min-h-11 items-center justify-center gap-2 rounded-md px-4 text-sm font-medium transition-colors pointer-fine:min-h-9 ${BUTTON_VARIANTS[variant]} ${className}`}
      {...props}
    />
  );
}

/** pill-tab — the filter chip strip. Active lifts by one surface notch. */
export function PillTab({
  active = false,
  children,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement> & { active?: boolean }) {
  return (
    <button
      role="tab"
      // Must always be present: `undefined` is dropped by React, leaving an
      // inactive tab with no selection state for assistive tech.
      aria-selected={active}
      className={`inline-flex min-h-11 items-center rounded-full px-3.5 text-sm transition-colors pointer-fine:min-h-8 ${
        active ? 'bg-elevated text-ink' : 'bg-transparent text-body hover:text-ink'
      }`}
      {...props}
    >
      {children}
    </button>
  );
}

/** keycap — the brand's only depth decoration. */
export function Keycap({ children }: { children: ReactNode }) {
  return (
    <kbd className="inline-flex h-5 items-center rounded-xs border border-hairline bg-gradient-to-b from-card to-surface px-1.5 font-sans text-[11px] leading-none text-body">
      {children}
    </kbd>
  );
}

export type ChipTone = 'info' | 'success' | 'danger' | 'warning';

const CHIP_TONES: Record<ChipTone, string> = {
  info: 'bg-accent-blue-soft text-accent-blue',
  success: 'bg-accent-green-soft text-accent-green',
  danger: 'bg-accent-red-soft text-accent-red',
  warning: 'bg-accent-yellow-soft text-accent-yellow',
};

/**
 * badge-info-soft — the only sanctioned use of a saturated accent.
 * Status, errors and the privacy notice all render through this.
 */
export function Chip({
  tone = 'info',
  children,
  className = '',
}: {
  tone?: ChipTone;
  children: ReactNode;
  className?: string;
}) {
  return (
    <span
      className={`inline-flex items-start gap-2 rounded-xs px-2 py-1 text-sm ${CHIP_TONES[tone]} ${className}`}
    >
      {children}
    </span>
  );
}

/** feature-card-dark — surface + hairline, tight padding (16-24px, never 32+). */
export function Card({
  children,
  className = '',
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <div className={`rounded-lg border border-hairline bg-surface ${className}`}>{children}</div>
  );
}
