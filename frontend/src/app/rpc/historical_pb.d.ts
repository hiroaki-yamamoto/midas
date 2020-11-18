import * as jspb from 'google-protobuf'

import * as entities_pb from './entities_pb';


export class HistChartProg extends jspb.Message {
  getSymbol(): string;
  setSymbol(value: string): HistChartProg;

  getNumSymbols(): number;
  setNumSymbols(value: number): HistChartProg;

  getCurSymbolNum(): number;
  setCurSymbolNum(value: number): HistChartProg;

  getNumObjects(): number;
  setNumObjects(value: number): HistChartProg;

  getCurObjectNum(): number;
  setCurObjectNum(value: number): HistChartProg;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HistChartProg.AsObject;
  static toObject(includeInstance: boolean, msg: HistChartProg): HistChartProg.AsObject;
  static serializeBinaryToWriter(message: HistChartProg, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HistChartProg;
  static deserializeBinaryFromReader(message: HistChartProg, reader: jspb.BinaryReader): HistChartProg;
}

export namespace HistChartProg {
  export type AsObject = {
    symbol: string,
    numSymbols: number,
    curSymbolNum: number,
    numObjects: number,
    curObjectNum: number,
  }
}

export class HistChartFetchReq extends jspb.Message {
  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): HistChartFetchReq;

  getSymbolsList(): Array<string>;
  setSymbolsList(value: Array<string>): HistChartFetchReq;
  clearSymbolsList(): HistChartFetchReq;
  addSymbols(value: string, index?: number): HistChartFetchReq;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HistChartFetchReq.AsObject;
  static toObject(includeInstance: boolean, msg: HistChartFetchReq): HistChartFetchReq.AsObject;
  static serializeBinaryToWriter(message: HistChartFetchReq, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HistChartFetchReq;
  static deserializeBinaryFromReader(message: HistChartFetchReq, reader: jspb.BinaryReader): HistChartFetchReq;
}

export namespace HistChartFetchReq {
  export type AsObject = {
    exchange: entities_pb.Exchanges,
    symbolsList: Array<string>,
  }
}

export class StopRequest extends jspb.Message {
  getExchangesList(): Array<entities_pb.Exchanges>;
  setExchangesList(value: Array<entities_pb.Exchanges>): StopRequest;
  clearExchangesList(): StopRequest;
  addExchanges(value: entities_pb.Exchanges, index?: number): StopRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StopRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StopRequest): StopRequest.AsObject;
  static serializeBinaryToWriter(message: StopRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StopRequest;
  static deserializeBinaryFromReader(message: StopRequest, reader: jspb.BinaryReader): StopRequest;
}

export namespace StopRequest {
  export type AsObject = {
    exchangesList: Array<entities_pb.Exchanges>,
  }
}

