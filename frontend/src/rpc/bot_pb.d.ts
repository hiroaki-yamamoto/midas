import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as entities_pb from './entities_pb';


export class Bot extends jspb.Message {
  getId(): string;
  setId(value: string): Bot;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): Bot;
  hasCreatedAt(): boolean;
  clearCreatedAt(): Bot;

  getName(): string;
  setName(value: string): Bot;

  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): Bot;

  getBaseCurrency(): string;
  setBaseCurrency(value: string): Bot;

  getTradingAmount(): string;
  setTradingAmount(value: string): Bot;

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
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    name: string,
    exchange: entities_pb.Exchanges,
    baseCurrency: string,
    tradingAmount: string,
    condition: string,
  }
}

export class Position extends jspb.Message {
  getId(): string;
  setId(value: string): Position;

  getBotid(): string;
  setBotid(value: string): Position;

  getSymbol(): string;
  setSymbol(value: string): Position;

  getStatus(): PositionStatus;
  setStatus(value: PositionStatus): Position;

  getTradingAmount(): string;
  setTradingAmount(value: string): Position;

  getValuation(): string;
  setValuation(value: string): Position;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Position.AsObject;
  static toObject(includeInstance: boolean, msg: Position): Position.AsObject;
  static serializeBinaryToWriter(message: Position, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Position;
  static deserializeBinaryFromReader(message: Position, reader: jspb.BinaryReader): Position;
}

export namespace Position {
  export type AsObject = {
    id: string,
    botid: string,
    symbol: string,
    status: PositionStatus,
    tradingAmount: string,
    valuation: string,
  }
}

export enum TriggerType { 
  MANUAL = 0,
}
export enum PositionStatus { 
  CLOSED = 0,
  OPENED = 1,
}
