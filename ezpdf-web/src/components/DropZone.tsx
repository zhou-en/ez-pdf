'use client';

import { useCallback, useRef, useState } from 'react';

/**
 * Drop target and file picker.
 *
 * Unlike the desktop app — where the webview drop event gives no hover
 * feedback — the browser does, so there is a real drag-over state here.
 */
export function DropZone({
  multiple,
  onFiles,
}: {
  multiple: boolean;
  onFiles: (files: File[]) => void;
}) {
  const [over, setOver] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const accept = useCallback(
    (list: FileList | null) => {
      if (!list) return;
      const pdfs = Array.from(list).filter(
        (f) => f.type === 'application/pdf' || /\.pdf$/i.test(f.name),
      );
      if (pdfs.length > 0) onFiles(multiple ? pdfs : pdfs.slice(0, 1));
    },
    [multiple, onFiles],
  );

  return (
    <>
      <button
        type="button"
        aria-label="Drop PDFs here or browse"
        onClick={() => inputRef.current?.click()}
        onDragOver={(e) => {
          e.preventDefault();
          setOver(true);
        }}
        onDragLeave={() => setOver(false)}
        onDrop={(e) => {
          e.preventDefault();
          setOver(false);
          accept(e.dataTransfer.files);
        }}
        className={`w-full rounded-lg border-2 border-dashed px-6 py-8 text-center transition-colors ${
          over
            ? 'border-accent-blue bg-accent-blue-soft text-accent-blue'
            : 'border-hairline text-mute hover:border-hairline-strong hover:text-body'
        }`}
      >
        <span className="block text-sm">
          Drop PDF{multiple ? 's' : ''} here
        </span>
        <span className="mt-1 block text-xs opacity-70">or click to browse</span>
      </button>
      <input
        ref={inputRef}
        type="file"
        accept="application/pdf,.pdf"
        multiple={multiple}
        hidden
        data-testid="file-input"
        onChange={(e) => {
          accept(e.target.files);
          e.target.value = '';
        }}
      />
    </>
  );
}
