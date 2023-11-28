import { z } from 'zod';

import { ApiKey } from './api-key.zod.ts';

export const ApiKeyList = z.object({
  keys: z.array(z.lazy(() => ApiKey)),
});
