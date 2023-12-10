import { z } from 'zod';

import { SymbolInfo } from './symbol-info.zod';

export const SymbolList = z.object({
  symbols: z.array(z.lazy(() => SymbolInfo)),
});

export type SymbolList = z.infer<typeof SymbolList>;
