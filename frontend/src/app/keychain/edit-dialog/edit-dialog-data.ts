import { ApiKey as APIKey } from '../../../rpc/api-key.zod';

export enum RespType {
  DELETE,
  POST,
  CANCEL,
}

export interface EditDialogData {
  type: RespType,
  index?: number,
  data?: APIKey;
}
