import { z } from 'zod';

import { Exchanges } from './exchanges.zod.ts';

export const ApiKey = z.object({
  exchange: z.lazy(() => Exchanges),
  id: z.string(),
  label: z.string(),
  prv_key: z.string(),
  pub_key: z.string(),
});

export type ApiKey = z.infer<typeof ApiKey>;
