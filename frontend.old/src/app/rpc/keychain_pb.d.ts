import * as jspb from 'google-protobuf'

import * as entities_pb from './entities_pb';


export class APIKey extends jspb.Message {
  getId(): string;
  setId(value: string): APIKey;

  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): APIKey;

  getLabel(): string;
  setLabel(value: string): APIKey;

  getPubKey(): string;
  setPubKey(value: string): APIKey;

  getPrvKey(): string;
  setPrvKey(value: string): APIKey;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): APIKey.AsObject;
  static toObject(includeInstance: boolean, msg: APIKey): APIKey.AsObject;
  static serializeBinaryToWriter(message: APIKey, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): APIKey;
  static deserializeBinaryFromReader(message: APIKey, reader: jspb.BinaryReader): APIKey;
}

export namespace APIKey {
  export type AsObject = {
    id: string,
    exchange: entities_pb.Exchanges,
    label: string,
    pubKey: string,
    prvKey: string,
  }
}

export class APIKeyList extends jspb.Message {
  getKeysList(): Array<APIKey>;
  setKeysList(value: Array<APIKey>): APIKeyList;
  clearKeysList(): APIKeyList;
  addKeys(value?: APIKey, index?: number): APIKey;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): APIKeyList.AsObject;
  static toObject(includeInstance: boolean, msg: APIKeyList): APIKeyList.AsObject;
  static serializeBinaryToWriter(message: APIKeyList, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): APIKeyList;
  static deserializeBinaryFromReader(message: APIKeyList, reader: jspb.BinaryReader): APIKeyList;
}

export namespace APIKeyList {
  export type AsObject = {
    keysList: Array<APIKey.AsObject>,
  }
}

export class APIRename extends jspb.Message {
  getLabel(): string;
  setLabel(value: string): APIRename;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): APIRename.AsObject;
  static toObject(includeInstance: boolean, msg: APIRename): APIRename.AsObject;
  static serializeBinaryToWriter(message: APIRename, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): APIRename;
  static deserializeBinaryFromReader(message: APIRename, reader: jspb.BinaryReader): APIRename;
}

export namespace APIRename {
  export type AsObject = {
    label: string,
  }
}

