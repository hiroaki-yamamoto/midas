import { z } from 'zod';

import { BotResponse } from './bot-response.zod';

export const BotList = z.object({
  bots: z.array(z.lazy(() => BotResponse)),
});

export type BotList = z.infer<typeof BotList>;
