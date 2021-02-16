import * as jspb from 'google-protobuf'



export class SymbolInfo extends jspb.Message {
  getSymbol(): string;
  setSymbol(value: string): SymbolInfo;

  getStatus(): string;
  setStatus(value: string): SymbolInfo;

  getBase(): string;
  setBase(value: string): SymbolInfo;

  getQuote(): string;
  setQuote(value: string): SymbolInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SymbolInfo.AsObject;
  static toObject(includeInstance: boolean, msg: SymbolInfo): SymbolInfo.AsObject;
  static serializeBinaryToWriter(message: SymbolInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SymbolInfo;
  static deserializeBinaryFromReader(message: SymbolInfo, reader: jspb.BinaryReader): SymbolInfo;
}

export namespace SymbolInfo {
  export type AsObject = {
    symbol: string,
    status: string,
    base: string,
    quote: string,
  }
}

export class Status extends jspb.Message {
  getCode(): number;
  setCode(value: number): Status;

  getMessage(): string;
  setMessage(value: string): Status;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Status.AsObject;
  static toObject(includeInstance: boolean, msg: Status): Status.AsObject;
  static serializeBinaryToWriter(message: Status, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Status;
  static deserializeBinaryFromReader(message: Status, reader: jspb.BinaryReader): Status;
}

export namespace Status {
  export type AsObject = {
    code: number,
    message: string,
  }
}

export class InsertOneResult extends jspb.Message {
  getId(): string;
  setId(value: string): InsertOneResult;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InsertOneResult.AsObject;
  static toObject(includeInstance: boolean, msg: InsertOneResult): InsertOneResult.AsObject;
  static serializeBinaryToWriter(message: InsertOneResult, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InsertOneResult;
  static deserializeBinaryFromReader(message: InsertOneResult, reader: jspb.BinaryReader): InsertOneResult;
}

export namespace InsertOneResult {
  export type AsObject = {
    id: string,
  }
}

export enum Exchanges { 
  UNKNOWN = 0,
  BINANCE = 1,
}
export enum BackTestPriceBase { 
  CLOSE = 0,
  OPEN = 1,
  HIGH = 2,
  LOW = 3,
  OPENCLOSEMID = 4,
  HIGHLOWMID = 5,
}
