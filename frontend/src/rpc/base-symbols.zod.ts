import { z } from 'zod';

export const BaseSymbols = z.object({
  symbols: z.array(z.string()),
});
