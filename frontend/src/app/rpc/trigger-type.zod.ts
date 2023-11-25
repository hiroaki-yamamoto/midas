import { z } from 'zod';

export const TriggerType = z.enum([
  'MANUAL',
]);
