import { z } from 'zod';

import { Exchanges } from './exchanges.zod';

export const BotRequest = z.object({
  baseCurrency: z.string(),
  condition: z.string(),
  exchange: z.lazy(() => Exchanges),
  id: z.string().optional(),
  name: z.string(),
  tradingAmount: z.string(),
});

export type BotRequest = z.infer<typeof BotRequest>;
