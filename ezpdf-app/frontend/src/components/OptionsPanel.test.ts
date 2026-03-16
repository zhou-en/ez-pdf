import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import OptionsPanel from './OptionsPanel.svelte';

describe('OptionsPanel rendering', () => {
  it('merge panel renders without op-specific inputs', () => {
    render(OptionsPanel, { op: 'merge' });
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    expect(screen.queryByRole('combobox')).not.toBeInTheDocument();
  });

  it('split panel shows mode radios and range input by default', () => {
    render(OptionsPanel, { op: 'split' });
    expect(screen.getByLabelText(/extract range/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/burst all pages/i)).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: /range/i })).toBeInTheDocument();
  });

  it('split panel hides range input when burst mode is selected', () => {
    render(OptionsPanel, { op: 'split', splitMode: 'burst' });
    expect(screen.queryByRole('textbox', { name: /range/i })).not.toBeInTheDocument();
  });

  it('remove panel shows pages text input', () => {
    render(OptionsPanel, { op: 'remove' });
    expect(screen.getByRole('textbox', { name: /pages/i })).toBeInTheDocument();
  });

  it('rotate panel shows degrees select and pages input', () => {
    render(OptionsPanel, { op: 'rotate' });
    expect(screen.getByRole('combobox', { name: /degrees/i })).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: /pages/i })).toBeInTheDocument();
  });

  it('reorder panel shows order text input', () => {
    render(OptionsPanel, { op: 'reorder' });
    expect(screen.getByRole('textbox', { name: /order/i })).toBeInTheDocument();
  });
});

describe('OptionsPanel input values', () => {
  it('range input reflects splitRange prop', () => {
    render(OptionsPanel, { op: 'split', splitRange: '2-4' });
    expect(screen.getByRole('textbox', { name: /range/i })).toHaveValue('2-4');
  });

  it('pages input reflects removePages prop', () => {
    render(OptionsPanel, { op: 'remove', removePages: '3,5' });
    expect(screen.getByRole('textbox', { name: /pages/i })).toHaveValue('3,5');
  });

  it('order input reflects reorderOrder prop', () => {
    render(OptionsPanel, { op: 'reorder', reorderOrder: '3,1,2' });
    expect(screen.getByRole('textbox', { name: /order/i })).toHaveValue('3,1,2');
  });

  it('typing in range input updates the displayed value', async () => {
    render(OptionsPanel, { op: 'split' });
    const input = screen.getByRole('textbox', { name: /range/i });
    await fireEvent.input(input, { target: { value: '1-3' } });
    expect(input).toHaveValue('1-3');
  });
});
