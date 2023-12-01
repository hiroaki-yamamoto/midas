import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';
import { Timestamp } from './timestamp.zod.ts';

export const Bot = z.object({
  base_currency: z.string(),
  condition: z.string(),
  created_at: z.lazy(() => Timestamp),
  exchange: z.lazy(() => Exchanges),
  id: z.string(),
  name: z.string(),
  trading_amount: z.string(),
});

export type Bot = z.infer<typeof Bot>;
