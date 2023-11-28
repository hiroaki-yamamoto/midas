import { z } from 'zod';

export const Status = z.object({
  code: z.number().max(4294967295).min(0),
  message: z.string(),
});

export type Status = z.infer<typeof Status>;
