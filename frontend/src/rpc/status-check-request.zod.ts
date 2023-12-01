import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';

export const StatusCheckRequest = z.object({
  exchange: z.lazy(() => Exchanges),
  symbol: z.string(),
});

export type StatusCheckRequest = z.infer<typeof StatusCheckRequest>;
