import { Injectable } from '@angular/core';
import { Exchanges } from '../rpc/entities_pb';
import { BookTicker, BookTickers } from '../rpc/bookticker_pb';
import { MidasSocket } from '../websocket';

@Injectable({
  providedIn: 'root'
})
export class TradeObserverService {
  public readonly binance: Map<string, BookTicker>;

  constructor() {
    this.binance = new Map();
  }
  private handle(exchange: string): (ev: MessageEvent<Blob>) => void {
    return (ev: MessageEvent<Blob>) => {
      ev.data.arrayBuffer().then((ab) => {
        const obj = BookTickers.deserializeBinary(new Uint8Array(ab));
        this[exchange] = new Map(obj.getBookTickerMapMap().toArray());
      });
    };
  }
  public connect() {
    const binanceBookTicker = new MidasSocket('/bookticker/binance');
    binanceBookTicker.addEventListener('message', this.handle('binance'));
  }
}
