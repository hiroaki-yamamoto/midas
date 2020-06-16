/**
 * @fileoverview gRPC-Web generated client stub for historical
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


import * as grpcWeb from 'grpc-web';

import * as entities_pb from './entities_pb';

import {
  HistChartFetchReq,
  HistChartProg} from './historical_pb';

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
    HistChartProg,
    (request: HistChartFetchReq) => {
      return request.serializeBinary();
    },
    HistChartProg.deserializeBinary
  );

  sync(
    request: HistChartFetchReq,
    metadata?: grpcWeb.Metadata) {
    return this.client_.serverStreaming(
      this.hostname_ +
        '/historical.HistChart/sync',
      request,
      metadata || {},
      this.methodInfosync);
  }

}

