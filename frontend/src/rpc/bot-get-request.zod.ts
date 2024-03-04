import { z } from 'zod';

import { SummaryDetail } from './summary-detail.zod';

export const BotGetRequest = z.object({
  granularity: z.lazy(() => SummaryDetail),
});

export type BotGetRequest = z.infer<typeof BotGetRequest>;
