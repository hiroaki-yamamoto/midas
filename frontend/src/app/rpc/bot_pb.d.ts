import * as jspb from 'google-protobuf'

import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class TargetIndicator extends jspb.Message {
  getAbsolute(): number;
  setAbsolute(value: number): TargetIndicator;

  getPercentage(): number;
  setPercentage(value: number): TargetIndicator;

  getCurrentprice(): google_protobuf_empty_pb.Empty | undefined;
  setCurrentprice(value?: google_protobuf_empty_pb.Empty): TargetIndicator;
  hasCurrentprice(): boolean;
  clearCurrentprice(): TargetIndicator;

  getCurrentvolume(): google_protobuf_duration_pb.Duration | undefined;
  setCurrentvolume(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasCurrentvolume(): boolean;
  clearCurrentvolume(): TargetIndicator;

  getVolumelasttick(): google_protobuf_duration_pb.Duration | undefined;
  setVolumelasttick(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasVolumelasttick(): boolean;
  clearVolumelasttick(): TargetIndicator;

  getHighpricelasttick(): google_protobuf_duration_pb.Duration | undefined;
  setHighpricelasttick(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasHighpricelasttick(): boolean;
  clearHighpricelasttick(): TargetIndicator;

  getLowpricelasttick(): google_protobuf_duration_pb.Duration | undefined;
  setLowpricelasttick(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasLowpricelasttick(): boolean;
  clearLowpricelasttick(): TargetIndicator;

  getMidpricelasttick(): google_protobuf_duration_pb.Duration | undefined;
  setMidpricelasttick(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasMidpricelasttick(): boolean;
  clearMidpricelasttick(): TargetIndicator;

  getOpenpricelasttick(): google_protobuf_duration_pb.Duration | undefined;
  setOpenpricelasttick(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasOpenpricelasttick(): boolean;
  clearOpenpricelasttick(): TargetIndicator;

  getClosepricelasttick(): google_protobuf_duration_pb.Duration | undefined;
  setClosepricelasttick(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasClosepricelasttick(): boolean;
  clearClosepricelasttick(): TargetIndicator;

  getSma(): google_protobuf_duration_pb.Duration | undefined;
  setSma(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasSma(): boolean;
  clearSma(): TargetIndicator;

  getEma(): google_protobuf_duration_pb.Duration | undefined;
  setEma(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasEma(): boolean;
  clearEma(): TargetIndicator;

  getRsi(): google_protobuf_duration_pb.Duration | undefined;
  setRsi(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasRsi(): boolean;
  clearRsi(): TargetIndicator;

  getMacd(): google_protobuf_duration_pb.Duration | undefined;
  setMacd(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasMacd(): boolean;
  clearMacd(): TargetIndicator;

  getCci(): google_protobuf_duration_pb.Duration | undefined;
  setCci(value?: google_protobuf_duration_pb.Duration): TargetIndicator;
  hasCci(): boolean;
  clearCci(): TargetIndicator;

  getTargetCase(): TargetIndicator.TargetCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TargetIndicator.AsObject;
  static toObject(includeInstance: boolean, msg: TargetIndicator): TargetIndicator.AsObject;
  static serializeBinaryToWriter(message: TargetIndicator, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TargetIndicator;
  static deserializeBinaryFromReader(message: TargetIndicator, reader: jspb.BinaryReader): TargetIndicator;
}

export namespace TargetIndicator {
  export type AsObject = {
    absolute: number,
    percentage: number,
    currentprice?: google_protobuf_empty_pb.Empty.AsObject,
    currentvolume?: google_protobuf_duration_pb.Duration.AsObject,
    volumelasttick?: google_protobuf_duration_pb.Duration.AsObject,
    highpricelasttick?: google_protobuf_duration_pb.Duration.AsObject,
    lowpricelasttick?: google_protobuf_duration_pb.Duration.AsObject,
    midpricelasttick?: google_protobuf_duration_pb.Duration.AsObject,
    openpricelasttick?: google_protobuf_duration_pb.Duration.AsObject,
    closepricelasttick?: google_protobuf_duration_pb.Duration.AsObject,
    sma?: google_protobuf_duration_pb.Duration.AsObject,
    ema?: google_protobuf_duration_pb.Duration.AsObject,
    rsi?: google_protobuf_duration_pb.Duration.AsObject,
    macd?: google_protobuf_duration_pb.Duration.AsObject,
    cci?: google_protobuf_duration_pb.Duration.AsObject,
  }

  export enum TargetCase { 
    TARGET_NOT_SET = 0,
    ABSOLUTE = 1,
    PERCENTAGE = 2,
    CURRENTPRICE = 3,
    CURRENTVOLUME = 4,
    VOLUMELASTTICK = 5,
    HIGHPRICELASTTICK = 6,
    LOWPRICELASTTICK = 7,
    MIDPRICELASTTICK = 8,
    OPENPRICELASTTICK = 9,
    CLOSEPRICELASTTICK = 10,
    SMA = 11,
    EMA = 12,
    RSI = 13,
    MACD = 14,
    CCI = 15,
  }
}

export class ConditionItem extends jspb.Message {
  getCmp(): CompareOp;
  setCmp(value: CompareOp): ConditionItem;

  getOpa(): TargetIndicator | undefined;
  setOpa(value?: TargetIndicator): ConditionItem;
  hasOpa(): boolean;
  clearOpa(): ConditionItem;

  getOpb(): TargetIndicator | undefined;
  setOpb(value?: TargetIndicator): ConditionItem;
  hasOpb(): boolean;
  clearOpb(): ConditionItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ConditionItem.AsObject;
  static toObject(includeInstance: boolean, msg: ConditionItem): ConditionItem.AsObject;
  static serializeBinaryToWriter(message: ConditionItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ConditionItem;
  static deserializeBinaryFromReader(message: ConditionItem, reader: jspb.BinaryReader): ConditionItem;
}

export namespace ConditionItem {
  export type AsObject = {
    cmp: CompareOp,
    opa?: TargetIndicator.AsObject,
    opb?: TargetIndicator.AsObject,
  }
}

export class Trigger extends jspb.Message {
  getAnd(): Trigger | undefined;
  setAnd(value?: Trigger): Trigger;
  hasAnd(): boolean;
  clearAnd(): Trigger;

  getOr(): Trigger | undefined;
  setOr(value?: Trigger): Trigger;
  hasOr(): boolean;
  clearOr(): Trigger;

  getNot(): Trigger | undefined;
  setNot(value?: Trigger): Trigger;
  hasNot(): boolean;
  clearNot(): Trigger;

  getSingle(): Trigger | undefined;
  setSingle(value?: Trigger): Trigger;
  hasSingle(): boolean;
  clearSingle(): Trigger;

  getTriggerCase(): Trigger.TriggerCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Trigger.AsObject;
  static toObject(includeInstance: boolean, msg: Trigger): Trigger.AsObject;
  static serializeBinaryToWriter(message: Trigger, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Trigger;
  static deserializeBinaryFromReader(message: Trigger, reader: jspb.BinaryReader): Trigger;
}

export namespace Trigger {
  export type AsObject = {
    and?: Trigger.AsObject,
    or?: Trigger.AsObject,
    not?: Trigger.AsObject,
    single?: Trigger.AsObject,
  }

  export enum TriggerCase { 
    TRIGGER_NOT_SET = 0,
    AND = 1,
    OR = 2,
    NOT = 3,
    SINGLE = 4,
  }
}

export class TriggerType extends jspb.Message {
  getManual(): Trigger | undefined;
  setManual(value?: Trigger): TriggerType;
  hasManual(): boolean;
  clearManual(): TriggerType;

  getTypeCase(): TriggerType.TypeCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TriggerType.AsObject;
  static toObject(includeInstance: boolean, msg: TriggerType): TriggerType.AsObject;
  static serializeBinaryToWriter(message: TriggerType, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TriggerType;
  static deserializeBinaryFromReader(message: TriggerType, reader: jspb.BinaryReader): TriggerType;
}

export namespace TriggerType {
  export type AsObject = {
    manual?: Trigger.AsObject,
  }

  export enum TypeCase { 
    TYPE_NOT_SET = 0,
    MANUAL = 1,
  }
}

export class Bot extends jspb.Message {
  getId(): string;
  setId(value: string): Bot;

  getName(): string;
  setName(value: string): Bot;

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

  getAutoReinvestment(): boolean;
  setAutoReinvestment(value: boolean): Bot;

  getTrigger(): TriggerType | undefined;
  setTrigger(value?: TriggerType): Bot;
  hasTrigger(): boolean;
  clearTrigger(): Bot;

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
    createdat?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    tradingamount: number,
    currentvaluation: number,
    realizedprofit: number,
    autoReinvestment: boolean,
    trigger?: TriggerType.AsObject,
  }
}

export enum CompareOp { 
  EQ = 0,
  GT = 1,
  GTE = 2,
  LT = 3,
  LTE = 4,
}
