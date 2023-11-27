import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';

export const Progress = z.object({
  cur: z.number().max(9223372036854775807).min(-9223372036854775808),
  exchange: z.lazy(() => Exchanges),
  size: z.number().max(9223372036854775807).min(-9223372036854775808),
  symbol: z.string(),
});
