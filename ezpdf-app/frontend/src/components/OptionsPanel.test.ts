import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import OptionsPanel from './OptionsPanel.svelte';

describe('OptionsPanel', () => {
  it('merge panel renders without op-specific inputs', () => {
    render(OptionsPanel, { op: 'merge' });
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    expect(screen.queryByRole('combobox')).not.toBeInTheDocument();
  });

  it('split panel shows mode toggle and range input', () => {
    render(OptionsPanel, { op: 'split' });
    expect(screen.getByLabelText(/extract range/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/burst all pages/i)).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: /range/i })).toBeInTheDocument();
  });

  it('remove panel shows pages text input', () => {
    render(OptionsPanel, { op: 'remove' });
    expect(screen.getByRole('textbox', { name: /pages/i })).toBeInTheDocument();
  });

  it('rotate panel shows degrees select and optional pages input', () => {
    render(OptionsPanel, { op: 'rotate' });
    expect(screen.getByRole('combobox', { name: /degrees/i })).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: /pages/i })).toBeInTheDocument();
  });

  it('reorder panel shows order text input', () => {
    render(OptionsPanel, { op: 'reorder' });
    expect(screen.getByRole('textbox', { name: /order/i })).toBeInTheDocument();
  });
});
