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

  getWatchprice(): google_protobuf_empty_pb.Empty | undefined;
  setWatchprice(value?: google_protobuf_empty_pb.Empty): TargetIndicator;
  hasWatchprice(): boolean;
  clearWatchprice(): TargetIndicator;

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
    watchprice?: google_protobuf_empty_pb.Empty.AsObject,
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
    WATCHPRICE = 4,
    CURRENTVOLUME = 5,
    VOLUMELASTTICK = 6,
    HIGHPRICELASTTICK = 7,
    LOWPRICELASTTICK = 8,
    MIDPRICELASTTICK = 9,
    OPENPRICELASTTICK = 10,
    CLOSEPRICELASTTICK = 11,
    SMA = 12,
    EMA = 13,
    RSI = 14,
    MACD = 15,
    CCI = 16,
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
  getAnd(): Triggers | undefined;
  setAnd(value?: Triggers): Trigger;
  hasAnd(): boolean;
  clearAnd(): Trigger;

  getOr(): Triggers | undefined;
  setOr(value?: Triggers): Trigger;
  hasOr(): boolean;
  clearOr(): Trigger;

  getNot(): Trigger | undefined;
  setNot(value?: Trigger): Trigger;
  hasNot(): boolean;
  clearNot(): Trigger;

  getSingle(): ConditionItem | undefined;
  setSingle(value?: ConditionItem): Trigger;
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
    and?: Triggers.AsObject,
    or?: Triggers.AsObject,
    not?: Trigger.AsObject,
    single?: ConditionItem.AsObject,
  }

  export enum TriggerCase { 
    TRIGGER_NOT_SET = 0,
    AND = 1,
    OR = 2,
    NOT = 3,
    SINGLE = 4,
  }
}

export class Triggers extends jspb.Message {
  getTriggersList(): Array<Trigger>;
  setTriggersList(value: Array<Trigger>): Triggers;
  clearTriggersList(): Triggers;
  addTriggers(value?: Trigger, index?: number): Trigger;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Triggers.AsObject;
  static toObject(includeInstance: boolean, msg: Triggers): Triggers.AsObject;
  static serializeBinaryToWriter(message: Triggers, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Triggers;
  static deserializeBinaryFromReader(message: Triggers, reader: jspb.BinaryReader): Triggers;
}

export namespace Triggers {
  export type AsObject = {
    triggersList: Array<Trigger.AsObject>,
  }
}

export class Trailing extends jspb.Message {
  getWatchpoint(): Trigger | undefined;
  setWatchpoint(value?: Trigger): Trailing;
  hasWatchpoint(): boolean;
  clearWatchpoint(): Trailing;

  getUnwatchpoint(): Trigger | undefined;
  setUnwatchpoint(value?: Trigger): Trailing;
  hasUnwatchpoint(): boolean;
  clearUnwatchpoint(): Trailing;

  getTriggerpoint(): Trigger | undefined;
  setTriggerpoint(value?: Trigger): Trailing;
  hasTriggerpoint(): boolean;
  clearTriggerpoint(): Trailing;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Trailing.AsObject;
  static toObject(includeInstance: boolean, msg: Trailing): Trailing.AsObject;
  static serializeBinaryToWriter(message: Trailing, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Trailing;
  static deserializeBinaryFromReader(message: Trailing, reader: jspb.BinaryReader): Trailing;
}

export namespace Trailing {
  export type AsObject = {
    watchpoint?: Trigger.AsObject,
    unwatchpoint?: Trigger.AsObject,
    triggerpoint?: Trigger.AsObject,
  }
}

export class Manual extends jspb.Message {
  getEntrypoint(): Trailing | undefined;
  setEntrypoint(value?: Trailing): Manual;
  hasEntrypoint(): boolean;
  clearEntrypoint(): Manual;

  getExitpoint(): Trailing | undefined;
  setExitpoint(value?: Trailing): Manual;
  hasExitpoint(): boolean;
  clearExitpoint(): Manual;

  getLosscutpoint(): Trigger | undefined;
  setLosscutpoint(value?: Trigger): Manual;
  hasLosscutpoint(): boolean;
  clearLosscutpoint(): Manual;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Manual.AsObject;
  static toObject(includeInstance: boolean, msg: Manual): Manual.AsObject;
  static serializeBinaryToWriter(message: Manual, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Manual;
  static deserializeBinaryFromReader(message: Manual, reader: jspb.BinaryReader): Manual;
}

export namespace Manual {
  export type AsObject = {
    entrypoint?: Trailing.AsObject,
    exitpoint?: Trailing.AsObject,
    losscutpoint?: Trigger.AsObject,
  }
}

export class TriggerType extends jspb.Message {
  getManual(): Manual | undefined;
  setManual(value?: Manual): TriggerType;
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
    manual?: Manual.AsObject,
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
    basecurrency: string,
    createdat?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    tradingamount: number,
    currentvaluation: number,
    realizedprofit: number,
    autoReinvestment: boolean,
    trigger?: TriggerType.AsObject,
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

export enum CompareOp { 
  EQ = 0,
  GT = 1,
  GTE = 2,
  LT = 3,
  LTE = 4,
}
export enum PositionStatus { 
  CLOSED = 0,
  OPENED = 1,
}
