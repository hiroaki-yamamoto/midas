import * as jspb from 'google-protobuf'



export class BotInfo extends jspb.Message {
  getId(): string;
  setId(value: string): BotInfo;

  getStrategy(): Strategy;
  setStrategy(value: Strategy): BotInfo;

  getName(): string;
  setName(value: string): BotInfo;

  getBasecurrency(): string;
  setBasecurrency(value: string): BotInfo;

  getDesc(): string;
  setDesc(value: string): BotInfo;

  getConfig(): string;
  setConfig(value: string): BotInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BotInfo.AsObject;
  static toObject(includeInstance: boolean, msg: BotInfo): BotInfo.AsObject;
  static serializeBinaryToWriter(message: BotInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BotInfo;
  static deserializeBinaryFromReader(message: BotInfo, reader: jspb.BinaryReader): BotInfo;
}

export namespace BotInfo {
  export type AsObject = {
    id: string,
    strategy: Strategy,
    name: string,
    basecurrency: string,
    desc: string,
    config: string,
  }
}

export class CurrentPosition extends jspb.Message {
  getId(): string;
  setId(value: string): CurrentPosition;

  getBotid(): string;
  setBotid(value: string): CurrentPosition;

  getSymbol(): string;
  setSymbol(value: string): CurrentPosition;

  getTradingamount(): number;
  setTradingamount(value: number): CurrentPosition;

  getProfitamount(): number;
  setProfitamount(value: number): CurrentPosition;

  getProfitpercent(): number;
  setProfitpercent(value: number): CurrentPosition;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CurrentPosition.AsObject;
  static toObject(includeInstance: boolean, msg: CurrentPosition): CurrentPosition.AsObject;
  static serializeBinaryToWriter(message: CurrentPosition, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CurrentPosition;
  static deserializeBinaryFromReader(message: CurrentPosition, reader: jspb.BinaryReader): CurrentPosition;
}

export namespace CurrentPosition {
  export type AsObject = {
    id: string,
    botid: string,
    symbol: string,
    tradingamount: number,
    profitamount: number,
    profitpercent: number,
  }
}

export class BotInfoList extends jspb.Message {
  getBotsList(): Array<BotInfo>;
  setBotsList(value: Array<BotInfo>): BotInfoList;
  clearBotsList(): BotInfoList;
  addBots(value?: BotInfo, index?: number): BotInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BotInfoList.AsObject;
  static toObject(includeInstance: boolean, msg: BotInfoList): BotInfoList.AsObject;
  static serializeBinaryToWriter(message: BotInfoList, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BotInfoList;
  static deserializeBinaryFromReader(message: BotInfoList, reader: jspb.BinaryReader): BotInfoList;
}

export namespace BotInfoList {
  export type AsObject = {
    botsList: Array<BotInfo.AsObject>,
  }
}

export class BotInfoListRequest extends jspb.Message {
  getOffset(): number;
  setOffset(value: number): BotInfoListRequest;

  getLimit(): number;
  setLimit(value: number): BotInfoListRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BotInfoListRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BotInfoListRequest): BotInfoListRequest.AsObject;
  static serializeBinaryToWriter(message: BotInfoListRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BotInfoListRequest;
  static deserializeBinaryFromReader(message: BotInfoListRequest, reader: jspb.BinaryReader): BotInfoListRequest;
}

export namespace BotInfoListRequest {
  export type AsObject = {
    offset: number,
    limit: number,
  }
}

export enum Strategy { 
  TRAILING = 0,
}
