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

export enum Exchanges { 
  BINANCE = 0,
}
