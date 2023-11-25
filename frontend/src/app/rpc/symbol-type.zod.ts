import { z } from 'zod';

export const SymbolType = z.enum([
  'Crypto',
  'Stock',
]);
