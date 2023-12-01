import { z } from 'zod';

export const Exchanges = z.enum([
  'Binance',
]);

export type Exchanges = z.infer<typeof Exchanges>;
