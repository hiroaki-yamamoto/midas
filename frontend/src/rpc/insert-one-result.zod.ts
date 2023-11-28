import { z } from 'zod';

export const InsertOneResult = z.object({
  id: z.string(),
});
