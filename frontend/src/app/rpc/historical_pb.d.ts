import * as jspb from 'google-protobuf'

import * as entities_pb from './entities_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';


export class HistChartProg extends jspb.Message {
  getId(): string;
  setId(value: string): HistChartProg;

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
    id: string,
    symbol: string,
    numSymbols: number,
    curSymbolNum: number,
    numObjects: number,
    curObjectNum: number,
  }
}

export class HistoryFetchRequest extends jspb.Message {
  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): HistoryFetchRequest;

  getSymbol(): string;
  setSymbol(value: string): HistoryFetchRequest;

  getStart(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setStart(value?: google_protobuf_timestamp_pb.Timestamp): HistoryFetchRequest;
  hasStart(): boolean;
  clearStart(): HistoryFetchRequest;

  getEnd(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setEnd(value?: google_protobuf_timestamp_pb.Timestamp): HistoryFetchRequest;
  hasEnd(): boolean;
  clearEnd(): HistoryFetchRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HistoryFetchRequest.AsObject;
  static toObject(includeInstance: boolean, msg: HistoryFetchRequest): HistoryFetchRequest.AsObject;
  static serializeBinaryToWriter(message: HistoryFetchRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HistoryFetchRequest;
  static deserializeBinaryFromReader(message: HistoryFetchRequest, reader: jspb.BinaryReader): HistoryFetchRequest;
}

export namespace HistoryFetchRequest {
  export type AsObject = {
    exchange: entities_pb.Exchanges,
    symbol: string,
    start?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    end?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

