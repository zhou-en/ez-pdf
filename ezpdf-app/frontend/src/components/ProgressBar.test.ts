import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import ProgressBar from './ProgressBar.svelte';

describe('ProgressBar', () => {
  it('renders the progress bar when visible is true', () => {
    render(ProgressBar, { visible: true });
    expect(screen.getByRole('progressbar')).toBeInTheDocument();
  });

  it('does not render when visible is false', () => {
    render(ProgressBar, { visible: false });
    expect(screen.queryByRole('progressbar')).not.toBeInTheDocument();
  });
});
