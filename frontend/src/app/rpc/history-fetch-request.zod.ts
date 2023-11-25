import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';

export const HistoryFetchRequest = z.object({
  end: z.string(),
  exchange: z.lazy(() => Exchanges),
  start: z.string(),
  symbol: z.string(),
});
