import * as jspb from 'google-protobuf'

import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb';
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

export class Condition extends jspb.Message {
  getAnd(): Condition | undefined;
  setAnd(value?: Condition): Condition;
  hasAnd(): boolean;
  clearAnd(): Condition;

  getOr(): Condition | undefined;
  setOr(value?: Condition): Condition;
  hasOr(): boolean;
  clearOr(): Condition;

  getNot(): Condition | undefined;
  setNot(value?: Condition): Condition;
  hasNot(): boolean;
  clearNot(): Condition;

  getSingle(): ConditionItem | undefined;
  setSingle(value?: ConditionItem): Condition;
  hasSingle(): boolean;
  clearSingle(): Condition;

  getConditionCase(): Condition.ConditionCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Condition.AsObject;
  static toObject(includeInstance: boolean, msg: Condition): Condition.AsObject;
  static serializeBinaryToWriter(message: Condition, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Condition;
  static deserializeBinaryFromReader(message: Condition, reader: jspb.BinaryReader): Condition;
}

export namespace Condition {
  export type AsObject = {
    and?: Condition.AsObject,
    or?: Condition.AsObject,
    not?: Condition.AsObject,
    single?: ConditionItem.AsObject,
  }

  export enum ConditionCase { 
    CONDITION_NOT_SET = 0,
    AND = 1,
    OR = 2,
    NOT = 3,
    SINGLE = 4,
  }
}

export enum CompareOp { 
  EQ = 0,
  GT = 1,
  GTE = 2,
  LT = 3,
  LTE = 4,
}
