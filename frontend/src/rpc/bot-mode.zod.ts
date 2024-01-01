import { z } from 'zod';

export const BotMode = z.enum([
  'BackTest',
  'ForwardTest',
  'RealPart',
]);

export type BotMode = z.infer<typeof BotMode>;
