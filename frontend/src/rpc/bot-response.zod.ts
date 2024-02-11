import { z } from 'zod';

import { BotMode } from './bot-mode.zod';
import { BotStatus } from './bot-status.zod';
import { Exchanges } from './exchanges.zod';
import { Timestamp } from './timestamp.zod';

export const BotResponse = z.object({
  baseCurrency: z.string(),
  condition: z.string(),
  createdAt: z.lazy(() => Timestamp),
  exchange: z.lazy(() => Exchanges),
  id: z.string(),
  mode: z.lazy(() => BotMode),
  name: z.string(),
  status: z.lazy(() => BotStatus),
  tradingAmount: z.string(),
});

export type BotResponse = z.infer<typeof BotResponse>;
