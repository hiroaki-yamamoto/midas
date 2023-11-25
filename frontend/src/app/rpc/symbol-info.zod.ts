import { z } from 'zod';

export const SymbolInfo = z.object({
  base: z.string(),
  base_commission_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  base_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  quote: z.string(),
  quote_commission_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  quote_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  status: z.string(),
  symbol: z.string(),
});
