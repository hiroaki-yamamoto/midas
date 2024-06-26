import { z } from 'zod';

import { ApiKey } from './api-key.zod';

export const ApiKeyList = z.object({
  keys: z.array(z.lazy(() => ApiKey)),
});

export type ApiKeyList = z.infer<typeof ApiKeyList>;
