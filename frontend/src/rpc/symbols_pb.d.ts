import * as jspb from 'google-protobuf'

import * as entities_pb from './entities_pb';


export class SymbolInfo extends jspb.Message {
  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): SymbolInfo;

  getType(): Type;
  setType(value: Type): SymbolInfo;

  getSymbol(): string;
  setSymbol(value: string): SymbolInfo;

  getStatus(): string;
  setStatus(value: string): SymbolInfo;

  getBase(): string;
  setBase(value: string): SymbolInfo;

  getQuote(): string;
  setQuote(value: string): SymbolInfo;

  getBasePrecision(): number;
  setBasePrecision(value: number): SymbolInfo;

  getQuotePrecision(): number;
  setQuotePrecision(value: number): SymbolInfo;

  getBaseCommissionPrecision(): number;
  setBaseCommissionPrecision(value: number): SymbolInfo;

  getQuoteCommissionPrecision(): number;
  setQuoteCommissionPrecision(value: number): SymbolInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SymbolInfo.AsObject;
  static toObject(includeInstance: boolean, msg: SymbolInfo): SymbolInfo.AsObject;
  static serializeBinaryToWriter(message: SymbolInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SymbolInfo;
  static deserializeBinaryFromReader(message: SymbolInfo, reader: jspb.BinaryReader): SymbolInfo;
}

export namespace SymbolInfo {
  export type AsObject = {
    exchange: entities_pb.Exchanges,
    type: Type,
    symbol: string,
    status: string,
    base: string,
    quote: string,
    basePrecision: number,
    quotePrecision: number,
    baseCommissionPrecision: number,
    quoteCommissionPrecision: number,
  }
}

export class SymbolList extends jspb.Message {
  getSymbolsList(): Array<SymbolInfo>;
  setSymbolsList(value: Array<SymbolInfo>): SymbolList;
  clearSymbolsList(): SymbolList;
  addSymbols(value?: SymbolInfo, index?: number): SymbolInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SymbolList.AsObject;
  static toObject(includeInstance: boolean, msg: SymbolList): SymbolList.AsObject;
  static serializeBinaryToWriter(message: SymbolList, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SymbolList;
  static deserializeBinaryFromReader(message: SymbolList, reader: jspb.BinaryReader): SymbolList;
}

export namespace SymbolList {
  export type AsObject = {
    symbolsList: Array<SymbolInfo.AsObject>,
  }
}

export class BaseSymbols extends jspb.Message {
  getSymbolsList(): Array<string>;
  setSymbolsList(value: Array<string>): BaseSymbols;
  clearSymbolsList(): BaseSymbols;
  addSymbols(value: string, index?: number): BaseSymbols;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BaseSymbols.AsObject;
  static toObject(includeInstance: boolean, msg: BaseSymbols): BaseSymbols.AsObject;
  static serializeBinaryToWriter(message: BaseSymbols, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BaseSymbols;
  static deserializeBinaryFromReader(message: BaseSymbols, reader: jspb.BinaryReader): BaseSymbols;
}

export namespace BaseSymbols {
  export type AsObject = {
    symbolsList: Array<string>,
  }
}

export enum Type { 
  CRYPTO = 0,
  STOCK = 1,
}
