import { z } from 'zod';

export const BotStatue = z.enum([
  'Running',
  'Stopped',
]);

export type BotStatue = z.infer<typeof BotStatue>;
