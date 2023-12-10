import { z } from 'zod';

import { PositionStatus } from './position-status.zod.ts';
import { Timestamp } from './timestamp.zod.ts';

export const Position = z.object({
  botId: z.string(),
  entryAt: z.lazy(() => Timestamp),
  exitAt: z.lazy(() => Timestamp).optional(),
  id: z.string(),
  profitAmount: z.string(),
  profitPercent: z.string(),
  status: z.lazy(() => PositionStatus),
  symbol: z.string(),
  tradingAmount: z.string(),
  valuation: z.string(),
});

export type Position = z.infer<typeof Position>;
