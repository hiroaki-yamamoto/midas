import { z } from 'zod';

export const ApiRename = z.object({
  label: z.string(),
});
