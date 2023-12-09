import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';
import { Timestamp } from './timestamp.zod.ts';

export const HistoryFetchRequest = z.object({
  end: z.lazy(() => Timestamp),
  exchange: z.lazy(() => Exchanges),
  start: z.lazy(() => Timestamp),
  symbol: z.string(),
});

export type HistoryFetchRequest = z.infer<typeof HistoryFetchRequest>;
