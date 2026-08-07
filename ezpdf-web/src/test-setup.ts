// jest-dom's `/vitest` entry imports `vitest` directly without depending on it,
// which pnpm's isolated node_modules correctly refuses to resolve. Registering
// the matchers ourselves keeps isolation strict.
import * as matchers from '@testing-library/jest-dom/matchers';
import { expect } from 'vitest';

expect.extend(matchers);
