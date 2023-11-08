import * as jspb from 'google-protobuf'



export class BookTicker extends jspb.Message {
  getId(): string;
  setId(value: string): BookTicker;

  getSymbol(): string;
  setSymbol(value: string): BookTicker;

  getBidPrice(): string;
  setBidPrice(value: string): BookTicker;

  getBidQty(): string;
  setBidQty(value: string): BookTicker;

  getAskPrice(): string;
  setAskPrice(value: string): BookTicker;

  getAskQty(): string;
  setAskQty(value: string): BookTicker;

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
    bidPrice: string,
    bidQty: string,
    askPrice: string,
    askQty: string,
  }
}

