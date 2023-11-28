import { z } from 'zod';

export const ApiRename = z.object({
  label: z.string(),
});

export type ApiRename = z.infer<typeof ApiRename>;
