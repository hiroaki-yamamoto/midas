import { z } from 'zod';

import { Bot } from './bot.zod';

export const BotList = z.object({
  bots: z.array(z.lazy(() => Bot)),
});

export type BotList = z.infer<typeof BotList>;
