/**
 * @fileoverview gRPC-Web generated client stub for historical
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


import * as grpcWeb from 'grpc-web';

import * as entities_pb from './entities_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';

import {
  HistChartFetchReq,
  HistChartProg,
  StopRequest} from './historical_pb';

export class HistChartClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: string; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: string; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'text';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname;
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodInfosync = new grpcWeb.AbstractClientBase.MethodInfo(
    google_protobuf_empty_pb.Empty,
    (request: HistChartFetchReq) => {
      return request.serializeBinary();
    },
    google_protobuf_empty_pb.Empty.deserializeBinary
  );

  sync(
    request: HistChartFetchReq,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: google_protobuf_empty_pb.Empty) => void) {
    return this.client_.rpcCall(
      this.hostname_ +
        '/historical.HistChart/sync',
      request,
      metadata || {},
      this.methodInfosync,
      callback);
  }

  methodInfosubscribe = new grpcWeb.AbstractClientBase.MethodInfo(
    HistChartProg,
    (request: google_protobuf_empty_pb.Empty) => {
      return request.serializeBinary();
    },
    HistChartProg.deserializeBinary
  );

  subscribe(
    request: google_protobuf_empty_pb.Empty,
    metadata?: grpcWeb.Metadata) {
    return this.client_.serverStreaming(
      this.hostname_ +
        '/historical.HistChart/subscribe',
      request,
      metadata || {},
      this.methodInfosubscribe);
  }

  methodInfostop = new grpcWeb.AbstractClientBase.MethodInfo(
    google_protobuf_empty_pb.Empty,
    (request: StopRequest) => {
      return request.serializeBinary();
    },
    google_protobuf_empty_pb.Empty.deserializeBinary
  );

  stop(
    request: StopRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: google_protobuf_empty_pb.Empty) => void) {
    return this.client_.rpcCall(
      this.hostname_ +
        '/historical.HistChart/stop',
      request,
      metadata || {},
      this.methodInfostop,
      callback);
  }

}

