import { z } from 'zod';

export const TestPriceBase = z.enum([
  'Close',
  'High',
  'HighLowMid',
  'Low',
  'Open',
  'OpenCloseMid',
]);
