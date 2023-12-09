import { APIKey } from '../../rpc/keychain_pb';

export enum RespType {
  DELETE,
  POST,
  CANCEL,
}

export interface EditDialogData {
  type: RespType,
  index?: number,
  data?: APIKey.AsObject;
}
