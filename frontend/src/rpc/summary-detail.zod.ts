import { z } from 'zod';

export const SummaryDetail = z.enum([
  'Detail',
  'Summary',
]);

export type SummaryDetail = z.infer<typeof SummaryDetail>;
