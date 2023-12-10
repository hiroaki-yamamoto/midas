import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';

export const ApiKey = z.object({
  exchange: z.lazy(() => Exchanges),
  id: z.string(),
  label: z.string(),
  prvKey: z.string(),
  pubKey: z.string(),
});

export type ApiKey = z.infer<typeof ApiKey>;
