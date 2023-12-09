import { z } from 'zod';

export const TriggerType = z.enum([
  'MANUAL',
]);

export type TriggerType = z.infer<typeof TriggerType>;
