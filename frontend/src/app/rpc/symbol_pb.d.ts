import * as jspb from 'google-protobuf'

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as entities_pb from './entities_pb';


export class RefreshRequest extends jspb.Message {
  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): RefreshRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RefreshRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RefreshRequest): RefreshRequest.AsObject;
  static serializeBinaryToWriter(message: RefreshRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RefreshRequest;
  static deserializeBinaryFromReader(message: RefreshRequest, reader: jspb.BinaryReader): RefreshRequest;
}

export namespace RefreshRequest {
  export type AsObject = {
    exchange: entities_pb.Exchanges,
  }
}

export class QueryRequest extends jspb.Message {
  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): QueryRequest;

  getStatus(): string;
  setStatus(value: string): QueryRequest;

  getSymbolsList(): Array<string>;
  setSymbolsList(value: Array<string>): QueryRequest;
  clearSymbolsList(): QueryRequest;
  addSymbols(value: string, index?: number): QueryRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): QueryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: QueryRequest): QueryRequest.AsObject;
  static serializeBinaryToWriter(message: QueryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): QueryRequest;
  static deserializeBinaryFromReader(message: QueryRequest, reader: jspb.BinaryReader): QueryRequest;
}

export namespace QueryRequest {
  export type AsObject = {
    exchange: entities_pb.Exchanges,
    status: string,
    symbolsList: Array<string>,
  }
}

export class QueryResponse extends jspb.Message {
  getSymbolsList(): Array<entities_pb.SymbolInfo>;
  setSymbolsList(value: Array<entities_pb.SymbolInfo>): QueryResponse;
  clearSymbolsList(): QueryResponse;
  addSymbols(value?: entities_pb.SymbolInfo, index?: number): entities_pb.SymbolInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): QueryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: QueryResponse): QueryResponse.AsObject;
  static serializeBinaryToWriter(message: QueryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): QueryResponse;
  static deserializeBinaryFromReader(message: QueryResponse, reader: jspb.BinaryReader): QueryResponse;
}

export namespace QueryResponse {
  export type AsObject = {
    symbolsList: Array<entities_pb.SymbolInfo.AsObject>,
  }
}

