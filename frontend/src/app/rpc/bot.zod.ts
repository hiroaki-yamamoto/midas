import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';

export const Bot = z.object({
  base_currency: z.string(),
  condition: z.string(),
  created_at: z.string(),
  exchange: z.lazy(() => Exchanges),
  id: z.string(),
  name: z.string(),
  trading_amount: z.string(),
});
