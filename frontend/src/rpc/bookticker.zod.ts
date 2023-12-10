import { z } from 'zod';

export const Bookticker = z.object({
  askPrice: z.string(),
  askQty: z.string(),
  bidPrice: z.string(),
  bidQty: z.string(),
  id: z.string(),
  symbol: z.string(),
});

export type Bookticker = z.infer<typeof Bookticker>;
