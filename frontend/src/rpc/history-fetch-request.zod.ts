import { z } from 'zod';

import { Exchanges } from './exchanges.zod';
import { Timestamp } from './timestamp.zod';

export const HistoryFetchRequest = z.object({
  end: z.lazy(() => Timestamp),
  exchange: z.lazy(() => Exchanges),
  start: z.lazy(() => Timestamp),
  symbol: z.string(),
});

export type HistoryFetchRequest = z.infer<typeof HistoryFetchRequest>;
