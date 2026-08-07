import { defineConfig } from 'vitest/config';
import path from 'node:path';

export default defineConfig({
  // Next.js compiles JSX itself; vitest needs to be told separately.
  esbuild: { jsx: 'automatic' },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/test-setup.ts'],
    include: ['src/**/*.test.{ts,tsx}'],
  },
  resolve: { alias: { '@': path.resolve(__dirname, './src') } },
});
