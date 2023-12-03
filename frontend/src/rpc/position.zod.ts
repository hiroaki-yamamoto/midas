import { z } from 'zod';

import { PositionStatus } from './position-status.zod.ts';

export const Position = z.object({
  bot_id: z.string(),
  id: z.string(),
  profit_amount: z.string(),
  profit_percent: z.string(),
  status: z.lazy(() => PositionStatus),
  symbol: z.string(),
  trading_amount: z.string(),
  valuation: z.string(),
});

export type Position = z.infer<typeof Position>;
