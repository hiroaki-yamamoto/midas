import { z } from 'zod';

export const TestPriceBase = z.enum([
  'Close',
  'High',
  'HighLowMid',
  'Low',
  'Open',
  'OpenCloseMid',
]);

export type TestPriceBase = z.infer<typeof TestPriceBase>;
