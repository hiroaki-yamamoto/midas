import { z } from 'zod';

export const BotStatus = z.enum([
  'Running',
  'Stopped',
]);

export type BotStatus = z.infer<typeof BotStatus>;
