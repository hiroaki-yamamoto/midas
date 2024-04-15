import { z } from 'zod';

import { Pagination } from './pagination.zod';

export const PositionQuery = z.object({
  demoMode: z.boolean(),
  pagination: z.lazy(() => Pagination),
});

export type PositionQuery = z.infer<typeof PositionQuery>;
