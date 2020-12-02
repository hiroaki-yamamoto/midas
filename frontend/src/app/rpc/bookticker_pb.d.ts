import * as jspb from 'google-protobuf'



export class BookTicker extends jspb.Message {
  getId(): string;
  setId(value: string): BookTicker;

  getSymbol(): string;
  setSymbol(value: string): BookTicker;

  getBidPrice(): number;
  setBidPrice(value: number): BookTicker;

  getBidQty(): number;
  setBidQty(value: number): BookTicker;

  getAskPrice(): number;
  setAskPrice(value: number): BookTicker;

  getAskQty(): number;
  setAskQty(value: number): BookTicker;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BookTicker.AsObject;
  static toObject(includeInstance: boolean, msg: BookTicker): BookTicker.AsObject;
  static serializeBinaryToWriter(message: BookTicker, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BookTicker;
  static deserializeBinaryFromReader(message: BookTicker, reader: jspb.BinaryReader): BookTicker;
}

export namespace BookTicker {
  export type AsObject = {
    id: string,
    symbol: string,
    bidPrice: number,
    bidQty: number,
    askPrice: number,
    askQty: number,
  }
}

