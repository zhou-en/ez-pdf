import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import OptionsPanel from './OptionsPanel.svelte';

describe('OptionsPanel rendering', () => {
  it('merge panel renders without op-specific inputs', () => {
    render(OptionsPanel, { op: 'merge' });
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    expect(screen.queryByRole('combobox')).not.toBeInTheDocument();
  });

  it('reorder panel renders nothing (no text input)', () => {
    render(OptionsPanel, { op: 'reorder' });
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    expect(screen.queryByRole('combobox')).not.toBeInTheDocument();
  });

  it('remove panel renders nothing (no text input)', () => {
    render(OptionsPanel, { op: 'remove' });
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    expect(screen.queryByRole('combobox')).not.toBeInTheDocument();
  });

  it('rotate panel shows degrees select only (no pages text input)', () => {
    render(OptionsPanel, { op: 'rotate' });
    expect(screen.getByRole('combobox', { name: /degrees/i })).toBeInTheDocument();
    expect(screen.queryByRole('textbox', { name: /pages/i })).not.toBeInTheDocument();
  });

  it('split panel shows combined/individual toggle (no range/burst radio)', () => {
    render(OptionsPanel, { op: 'split' });
    // Should show combined/individual toggle
    expect(screen.getByLabelText(/combined/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/individual/i)).toBeInTheDocument();
    // Should NOT show old range/burst radios or text input
    expect(screen.queryByLabelText(/extract range/i)).not.toBeInTheDocument();
    expect(screen.queryByLabelText(/burst all pages/i)).not.toBeInTheDocument();
    expect(screen.queryByRole('textbox', { name: /range/i })).not.toBeInTheDocument();
  });

  it('split combined is selected by default', () => {
    render(OptionsPanel, { op: 'split', splitOutputMode: 'combined' });
    const combinedRadio = screen.getByLabelText(/combined/i) as HTMLInputElement;
    expect(combinedRadio.checked).toBe(true);
  });

  it('split individual radio selects individual mode', async () => {
    render(OptionsPanel, { op: 'split' });
    const individualRadio = screen.getByLabelText(/individual/i) as HTMLInputElement;
    await fireEvent.change(individualRadio);
    expect(individualRadio.checked).toBe(true);
  });
});

describe('OptionsPanel input values', () => {
  it('degrees combobox works for rotate op', async () => {
    render(OptionsPanel, { op: 'rotate', rotateDegrees: 90 });
    const select = screen.getByRole('combobox', { name: /degrees/i }) as HTMLSelectElement;
    expect(select.value).toBe('90');
  });
});
