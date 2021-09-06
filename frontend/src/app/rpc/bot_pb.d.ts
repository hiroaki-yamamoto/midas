import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as entities_pb from './entities_pb';


export class Bot extends jspb.Message {
  getId(): string;
  setId(value: string): Bot;

  getName(): string;
  setName(value: string): Bot;

  getExchange(): entities_pb.Exchanges;
  setExchange(value: entities_pb.Exchanges): Bot;

  getCreatedat(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedat(value?: google_protobuf_timestamp_pb.Timestamp): Bot;
  hasCreatedat(): boolean;
  clearCreatedat(): Bot;

  getTradeAmount(): number;
  setTradeAmount(value: number): Bot;

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
    exchange: entities_pb.Exchanges,
    createdat?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    tradeAmount: number,
    reinvest: boolean,
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

  getTradingamount(): number;
  setTradingamount(value: number): Position;

  getValuation(): number;
  setValuation(value: number): Position;

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
    tradingamount: number,
    valuation: number,
  }
}

export enum TriggerType { 
  MANUAL = 0,
}
export enum PositionStatus { 
  CLOSED = 0,
  OPENED = 1,
}
