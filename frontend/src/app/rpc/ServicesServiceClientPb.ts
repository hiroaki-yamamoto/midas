/**
 * @fileoverview gRPC-Web generated client stub for services
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


import * as grpcWeb from 'grpc-web';

import {
  BotInfoList,
  BotInfoListRequest} from './services_pb';

export class BotManagerClient {
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

  methodInfoListBotInfo = new grpcWeb.AbstractClientBase.MethodInfo(
    BotInfoList,
    (request: BotInfoListRequest) => {
      return request.serializeBinary();
    },
    BotInfoList.deserializeBinary
  );

  listBotInfo(
    request: BotInfoListRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: BotInfoList) => void) {
    return this.client_.rpcCall(
      this.hostname_ +
        '/services.BotManager/ListBotInfo',
      request,
      metadata || {},
      this.methodInfoListBotInfo,
      callback);
  }

}

