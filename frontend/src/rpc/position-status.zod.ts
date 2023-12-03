import { z } from 'zod';

export const PositionStatus = z.enum([
  'CLOSE',
  'OPEN',
]);

export type PositionStatus = z.infer<typeof PositionStatus>;
