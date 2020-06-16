import * as jspb from "google-protobuf"

import * as entities_pb from './entities_pb';

export class HistChartProg extends jspb.Message {
  getSymbol(): string;
  setSymbol(value: string): void;

  getNumSymbols(): number;
  setNumSymbols(value: number): void;

  getCurSymbolNum(): number;
  setCurSymbolNum(value: number): void;

  getNumObjects(): number;
  setNumObjects(value: number): void;

  getCurObjectNum(): number;
  setCurObjectNum(value: number): void;

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
  setExchange(value: entities_pb.Exchanges): void;

  getSymbol(): string;
  setSymbol(value: string): void;

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
    symbol: string,
  }
}

