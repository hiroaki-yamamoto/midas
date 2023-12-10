import { z } from 'zod';

import { Exchanges } from './exchanges.zod';
import { SymbolType } from './symbol-type.zod';

export const SymbolInfo = z.object({
  base: z.string(),
  baseCommissionPrecision: z.number().max(9223372036854775807).min(-9223372036854775808),
  basePrecision: z.number().max(9223372036854775807).min(-9223372036854775808),
  exchange: z.lazy(() => Exchanges),
  quote: z.string(),
  quoteCommissionPrecision: z.number().max(9223372036854775807).min(-9223372036854775808),
  quotePrecision: z.number().max(9223372036854775807).min(-9223372036854775808),
  status: z.string(),
  symbol: z.string(),
  symbolType: z.lazy(() => SymbolType),
});

export type SymbolInfo = z.infer<typeof SymbolInfo>;
