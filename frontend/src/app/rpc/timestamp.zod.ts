import { z } from 'zod';

export const Timestamp = z.object({
  nanos: z.number().max(4294967295).min(0),
  secs: z.number().max(9223372036854775807).min(-9223372036854775808),
});
