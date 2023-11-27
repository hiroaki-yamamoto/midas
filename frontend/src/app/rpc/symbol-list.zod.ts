import { z } from 'zod';

import { SymbolInfo } from './symbol-info.zod.ts';

export const SymbolList = z.object({
  symbols: z.array(z.lazy(() => SymbolInfo)),
});
