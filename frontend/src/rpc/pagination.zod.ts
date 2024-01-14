import { z } from 'zod';

export const Pagination = z.object({
  limit: z.number().int().max(9223372036854775807).min(-9223372036854775808),
  offset: z.number().int().max(18446744073709551615).min(0),
});

export type Pagination = z.infer<typeof Pagination>;
