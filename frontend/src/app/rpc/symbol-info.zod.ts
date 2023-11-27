import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';
import { SymbolType } from './symbol-type.zod.ts';

export const SymbolInfo = z.object({
  base: z.string(),
  base_commission_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  base_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  exchange: z.lazy(() => Exchanges),
  quote: z.string(),
  quote_commission_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  quote_precision: z.number().max(9223372036854775807).min(-9223372036854775808),
  status: z.string(),
  symbol: z.string(),
  symbol_type: z.lazy(() => SymbolType),
});
