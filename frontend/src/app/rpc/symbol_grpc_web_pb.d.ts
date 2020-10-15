import * as grpcWeb from 'grpc-web';

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as symbol_pb from './symbol_pb';


export class SymbolClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  refresh(
    request: symbol_pb.RefreshRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  query(
    request: symbol_pb.QueryRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: symbol_pb.QueryResponse) => void
  ): grpcWeb.ClientReadableStream<symbol_pb.QueryResponse>;

}

export class SymbolPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  refresh(
    request: symbol_pb.RefreshRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  query(
    request: symbol_pb.QueryRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<symbol_pb.QueryResponse>;

}

