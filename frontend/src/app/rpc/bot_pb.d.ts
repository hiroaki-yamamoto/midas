import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';


export class Bot extends jspb.Message {
  getId(): string;
  setId(value: string): Bot;

  getName(): string;
  setName(value: string): Bot;

  getBasecurrency(): string;
  setBasecurrency(value: string): Bot;

  getCreatedat(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedat(value?: google_protobuf_timestamp_pb.Timestamp): Bot;
  hasCreatedat(): boolean;
  clearCreatedat(): Bot;

  getTradingamount(): number;
  setTradingamount(value: number): Bot;

  getCurrentvaluation(): number;
  setCurrentvaluation(value: number): Bot;

  getRealizedprofit(): number;
  setRealizedprofit(value: number): Bot;

  getReinvest(): boolean;
  setReinvest(value: boolean): Bot;

  getCondition(): string;
  setCondition(value: string): Bot;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Bot.AsObject;
  static toObject(includeInstance: boolean, msg: Bot): Bot.AsObject;
  static serializeBinaryToWriter(message: Bot, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Bot;
  static deserializeBinaryFromReader(message: Bot, reader: jspb.BinaryReader): Bot;
}

export namespace Bot {
  export type AsObject = {
    id: string,
    name: string,
    basecurrency: string,
    createdat?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    tradingamount: number,
    currentvaluation: number,
    realizedprofit: number,
    reinvest: boolean,
    condition: string,
  }
}

