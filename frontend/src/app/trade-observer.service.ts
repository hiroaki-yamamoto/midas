import { Message } from '@angular/compiler/src/i18n/i18n_ast';
import { Injectable } from '@angular/core';
import { Exchanges } from './rpc/entities_pb';
import { MidasSocket } from './websocket';

export interface BookTicker {
  id: string;
  exchange: Exchanges;
  symbol: string;
  bid_price: number;
  bid_qty: number;
  ask_price: number;
  ask_qty: number;
}

@Injectable({
  providedIn: 'root'
})
export class TradeObserverService {
  public data: {[key: string]: [BookTicker]};

  constructor() {
    this.data = {};
  }
  private handle(exchange: string): (ev: MessageEvent) => void {
    return function(ev: MessageEvent) {
      const obj = JSON.parse(ev.data) as BookTicker;
      const index = this.data[exchange] && this.data[exchange].findIndex((el: BookTicker) => {
        return el.symbol === obj.symbol;
      }) || -1;
      if (index < 0) {
        this.data[exchange].push(obj);
      } else {
        this.data[exchange][index] = obj;
      }
    };
  }
  public connect() {
    const binanceBookTicker = new MidasSocket('/bookticker');
    binanceBookTicker.addEventListener('message', this.handle('binance'));
  }
}
