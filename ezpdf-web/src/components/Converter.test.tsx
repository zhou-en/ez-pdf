import { act, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { Converter } from './Converter';
import { runOperation, type OpResult } from '@/lib/operations';

// The real implementation pulls in a wasm module. None of the copy behaviour
// under test depends on it, so the operation is stubbed and only its result
// shape matters.
vi.mock('@/lib/operations', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@/lib/operations')>()),
  runOperation: vi.fn(),
}));

// Longer than the 1500-character slice the preview renders, so a copy that
// grabbed the preview instead of the document would fail this test.
const MARKDOWN = `# Heading\n\n${'x'.repeat(3000)}`;

function pdfFile(name = 'a.pdf') {
  const bytes = new Uint8Array([0x25, 0x50, 0x44, 0x46]);
  const file = new File([bytes], name, { type: 'application/pdf' });
  // jsdom's File implements neither arrayBuffer() nor Blob streaming, and
  // Converter reads the bytes that way.
  Object.defineProperty(file, 'arrayBuffer', {
    value: async () => bytes.buffer,
  });
  return file;
}

async function convertTo(result: OpResult) {
  vi.mocked(runOperation).mockResolvedValue(result);
  render(<Converter userId="u1" />);
  await userEvent.upload(screen.getByTestId('file-input'), [pdfFile()]);
  await userEvent.click(screen.getByRole('button', { name: /^Run Markdown$/ }));
  await waitFor(() => expect(screen.getByText(result.filename)).toBeInTheDocument());
}

describe('copying a markdown result', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: { writeText: vi.fn().mockResolvedValue(undefined) },
    });
  });

  it('puts the whole document on the clipboard, not the truncated preview', async () => {
    await convertTo({ data: MARKDOWN, filename: 'a.md', preview: MARKDOWN });

    await userEvent.click(screen.getByRole('button', { name: 'Copy' }));

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(MARKDOWN);
  });

  it('confirms the copy, then returns to an actionable label', async () => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    try {
      await convertTo({ data: MARKDOWN, filename: 'a.md', preview: MARKDOWN });

      await userEvent.click(screen.getByRole('button', { name: 'Copy' }));
      await waitFor(() => expect(screen.getByRole('button', { name: 'Copied' })).toBeTruthy());

      // Wrapped because the revert is driven by a timer, not by an event.
      await act(async () => {
        await vi.advanceTimersByTimeAsync(2000);
      });
      expect(screen.getByRole('button', { name: 'Copy' })).toBeTruthy();
    } finally {
      vi.useRealTimers();
    }
  });

  it('offers no copy button for a PDF result — bytes are not clipboard content', async () => {
    await convertTo({ data: new Uint8Array([1, 2, 3]), filename: 'a-merged.pdf' });

    expect(screen.queryByRole('button', { name: 'Copy' })).toBeNull();
  });
});
