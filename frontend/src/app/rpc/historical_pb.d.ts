import * as jspb from 'google-protobuf'

import * as entities_pb from './entities_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';


export class Progress extends jspb.Message {
  getSize(): number;
  setSize(value: number): Progress;

  getCur(): number;
  setCur(value: number): Progress;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Progress.AsObject;
  static toObject(includeInstance: boolean, msg: Progress): Progress.AsObject;
  static serializeBinaryToWriter(message: Progress, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Progress;
  static deserializeBinaryFromReader(message: Progress, reader: jspb.BinaryReader): Progress;
}

export namespace Progress {
  export type AsObject = {
    size: number,
    cur: number,
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

