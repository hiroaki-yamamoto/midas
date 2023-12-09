import { z } from 'zod';

import { PositionStatus } from './position-status.zod.ts';
import { Timestamp } from './timestamp.zod.ts';

export const Position = z.object({
  bot_id: z.string(),
  entry_at: z.lazy(() => Timestamp),
  exit_at: z.lazy(() => Timestamp).optional(),
  id: z.string(),
  profit_amount: z.string(),
  profit_percent: z.string(),
  status: z.lazy(() => PositionStatus),
  symbol: z.string(),
  trading_amount: z.string(),
  valuation: z.string(),
});

export type Position = z.infer<typeof Position>;
