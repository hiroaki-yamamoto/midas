import { z } from 'zod';

export const BotMode = z.enum([
  'BackTest',
  'ForwardTest',
  'Live',
]);

export type BotMode = z.infer<typeof BotMode>;
