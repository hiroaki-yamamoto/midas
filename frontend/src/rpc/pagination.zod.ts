import { z } from 'zod';

export const Pagination = z.object({
  id: z.string().optional(),
  limit: z.number().int().max(9223372036854775807).min(-9223372036854775808),
});

export type Pagination = z.infer<typeof Pagination>;
