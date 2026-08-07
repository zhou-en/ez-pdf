import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { DropZone } from './DropZone';

function pdf(name = 'a.pdf') {
  return new File(['%PDF-1.5'], name, { type: 'application/pdf' });
}

describe('DropZone', () => {
  it('accepts PDFs from the picker', async () => {
    const onFiles = vi.fn();
    render(<DropZone multiple onFiles={onFiles} />);
    await userEvent.upload(screen.getByTestId('file-input'), [pdf('a.pdf'), pdf('b.pdf')]);
    expect(onFiles).toHaveBeenCalledWith([
      expect.objectContaining({ name: 'a.pdf' }),
      expect.objectContaining({ name: 'b.pdf' }),
    ]);
  });

  it('filters out non-PDFs', async () => {
    const onFiles = vi.fn();
    render(<DropZone multiple onFiles={onFiles} />);
    const txt = new File(['hi'], 'notes.txt', { type: 'text/plain' });
    await userEvent.upload(screen.getByTestId('file-input'), [txt, pdf()]);
    expect(onFiles).toHaveBeenCalledWith([expect.objectContaining({ name: 'a.pdf' })]);
  });

  it('does not fire when every file is rejected', async () => {
    const onFiles = vi.fn();
    render(<DropZone multiple onFiles={onFiles} />);
    const txt = new File(['hi'], 'notes.txt', { type: 'text/plain' });
    await userEvent.upload(screen.getByTestId('file-input'), [txt]);
    expect(onFiles).not.toHaveBeenCalled();
  });

  it('keeps only the first file when multiple is false', async () => {
    const onFiles = vi.fn();
    render(<DropZone multiple={false} onFiles={onFiles} />);
    await userEvent.upload(screen.getByTestId('file-input'), [pdf('a.pdf'), pdf('b.pdf')]);
    expect(onFiles).toHaveBeenCalledWith([expect.objectContaining({ name: 'a.pdf' })]);
  });

  it('accepts a .pdf name even when the browser reports no MIME type', async () => {
    const onFiles = vi.fn();
    render(<DropZone multiple onFiles={onFiles} />);
    await userEvent.upload(
      screen.getByTestId('file-input'),
      [new File(['%PDF'], 'weird.pdf', { type: '' })],
    );
    expect(onFiles).toHaveBeenCalled();
  });

  it('labels itself for assistive tech', () => {
    render(<DropZone multiple onFiles={vi.fn()} />);
    expect(screen.getByRole('button', { name: /drop pdfs here or browse/i })).toBeInTheDocument();
  });
});
