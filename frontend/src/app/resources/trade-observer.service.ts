import { Injectable } from '@angular/core';
import { BookTicker } from '../rpc/bookticker_pb';
import { MidasSocket } from '../websocket';
import { decodeAsync } from '@msgpack/msgpack';

type BookTickers = {[symbol: string]: BookTicker.AsObject};

@Injectable({
  providedIn: 'root'
})
export class TradeObserverService {
  public readonly binance: BookTickers;

  constructor() {
    this.binance = {};
  }
  private handle(exchange: string): (ev: MessageEvent<Blob>) => void {
    return (ev: MessageEvent<Blob>) => {
      decodeAsync(ev.data.stream()).then(
        (obj: BookTicker.AsObject) => {
          this[exchange] = Object.assign(this[exchange], obj);
        }
      );
    };
  }
  public connect() {
    const binanceBookTicker = new MidasSocket('/bookticker/binance');
    binanceBookTicker.addEventListener('message', this.handle('binance'));
  }
}
