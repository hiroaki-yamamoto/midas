import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';
import { Timestamp } from './timestamp.zod.ts';

export const Bot = z.object({
  baseCurrency: z.string(),
  condition: z.string(),
  createdAt: z.lazy(() => Timestamp).optional(),
  exchange: z.lazy(() => Exchanges),
  id: z.string().optional(),
  name: z.string(),
  tradingAmount: z.string(),
});

export type Bot = z.infer<typeof Bot>;
