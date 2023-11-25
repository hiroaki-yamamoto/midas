import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';

export const Progress = z.object({
  cur: z.string(),
  exchange: z.lazy(() => Exchanges),
  size: z.string(),
  symbol: z.string(),
});
