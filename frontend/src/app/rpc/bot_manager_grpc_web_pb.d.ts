import * as grpcWeb from 'grpc-web';

import * as bot_manager_pb from './bot_manager_pb';


export class BotManagerClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  listBotInfo(
    request: bot_manager_pb.BotInfoListRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.Error,
               response: bot_manager_pb.BotInfoList) => void
  ): grpcWeb.ClientReadableStream<bot_manager_pb.BotInfoList>;

}

export class BotManagerPromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  listBotInfo(
    request: bot_manager_pb.BotInfoListRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<bot_manager_pb.BotInfoList>;

}

