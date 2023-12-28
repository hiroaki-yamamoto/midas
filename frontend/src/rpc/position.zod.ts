import { z } from 'zod';

import { PositionStatus } from './position-status.zod';
import { Timestamp } from './timestamp.zod';

export const Position = z.object({
  amount: z.string(),
  botId: z.string(),
  entryAt: z.lazy(() => Timestamp),
  entryPrice: z.string(),
  exitAt: z.lazy(() => Timestamp).optional(),
  exitPrice: z.string().optional(),
  id: z.string(),
  status: z.lazy(() => PositionStatus),
  symbol: z.string(),
});

export type Position = z.infer<typeof Position>;
