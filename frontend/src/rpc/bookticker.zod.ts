import { z } from 'zod';

export const Bookticker = z.object({
  ask_price: z.string(),
  ask_qty: z.string(),
  bid_price: z.string(),
  bid_qty: z.string(),
  id: z.string(),
  symbol: z.string(),
});

export type Bookticker = z.infer<typeof Bookticker>;
