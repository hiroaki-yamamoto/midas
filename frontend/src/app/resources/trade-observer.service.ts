import { Injectable } from '@angular/core';
import { Exchanges } from '../rpc/entities_pb';
import { BookTicker } from '../rpc/bookticker_pb';
import { MidasSocket } from '../websocket';

@Injectable({
  providedIn: 'root'
})
export class TradeObserverService {
  public data: {[key: string]: [BookTicker]};

  constructor() {
    this.data = {};
  }
  private handle(exchange: string): (ev: MessageEvent<Blob>) => void {
    return (ev: MessageEvent<Blob>) => {
      console.log(ev.data);
      ev.data.arrayBuffer().then((ab) => {
        const obj = BookTicker.deserializeBinary(new Uint8Array(ab));
        const index = (this.data[exchange]) ? this.data[exchange].findIndex((el: BookTicker) => {
          return el.getSymbol() === obj.getSymbol();
        }) : -1;
        if (index < 0) {
          this.data[exchange] = [obj];
        } else {
          this.data[exchange][index] = obj;
        }
      });
    };
  }
  public connect() {
    const binanceBookTicker = new MidasSocket('/bookticker/binance');
    binanceBookTicker.addEventListener('message', this.handle('binance'));
  }
}
