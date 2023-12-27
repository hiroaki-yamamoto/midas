import { z } from 'zod';

export const Timestamp = z.object({
  nanos: z.number().int().max(4294967295).min(0),
  secs: z.number().int().max(9223372036854775807).min(-9223372036854775808),
});

export type Timestamp = z.infer<typeof Timestamp>;
