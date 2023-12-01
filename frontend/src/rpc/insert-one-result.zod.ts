import { z } from 'zod';

export const InsertOneResult = z.object({
  id: z.string(),
});

export type InsertOneResult = z.infer<typeof InsertOneResult>;
