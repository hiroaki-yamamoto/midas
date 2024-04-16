import { z } from 'zod';

import { Position } from './position.zod';

export const PositionList = z.object({
  positions: z.array(z.lazy(() => Position)),
});

export type PositionList = z.infer<typeof PositionList>;
