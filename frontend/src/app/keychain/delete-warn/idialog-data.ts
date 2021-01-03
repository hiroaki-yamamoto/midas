import { APIKey } from '../../rpc/keychain_pb';
export interface IDialogData {
  index: number,
  data: APIKey.AsObject,
}
