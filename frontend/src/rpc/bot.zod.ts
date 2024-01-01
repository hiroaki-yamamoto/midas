import { z } from 'zod';

import { BotMode } from './bot-mode.zod';
import { BotStatus } from './bot-status.zod';
import { Exchanges } from './exchanges.zod';
import { Timestamp } from './timestamp.zod';

export const Bot = z.object({
  baseCurrency: z.string(),
  condition: z.string().optional(),
  createdAt: z.lazy(() => Timestamp).optional(),
  exchange: z.lazy(() => Exchanges),
  id: z.string().optional(),
  mode: z.lazy(() => BotMode),
  name: z.string(),
  status: z.lazy(() => BotStatus),
  tradingAmount: z.string(),
});

export type Bot = z.infer<typeof Bot>;
