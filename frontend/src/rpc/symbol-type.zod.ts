import { z } from 'zod';

export const SymbolType = z.enum([
  'Crypto',
  'Stock',
]);

export type SymbolType = z.infer<typeof SymbolType>;
