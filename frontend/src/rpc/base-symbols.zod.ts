import { z } from 'zod';

export const BaseSymbols = z.object({
  symbols: z.array(z.string()),
});

export type BaseSymbols = z.infer<typeof BaseSymbols>;
