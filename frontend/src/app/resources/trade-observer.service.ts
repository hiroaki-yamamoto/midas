import { Injectable } from '@angular/core';
import { Bookticker as BookTicker } from '../../rpc/bookticker.zod';
import { MidasSocket } from '../websocket';

type BookTickers = { [symbol: string]: BookTicker; };

@Injectable({
  providedIn: 'root'
})
export class TradeObserverService {
  public readonly binance: BookTickers;
  public onChanged?: (exchange: string) => void;

  constructor() {
    this.binance = {};
  }
  private handle(exchange: string): (ev: MessageEvent<string>) => void {
    return (ev: MessageEvent<string>) => {
      const entries = Object.entries(JSON.parse(ev.data))
        .map(([symbol, bookticker]) => {
          return [symbol, BookTicker.parse(bookticker)];
        });
      const obj: { [key: string]: BookTicker; } = Object.fromEntries(entries);
      this[exchange] = Object.assign(this[exchange], obj);
      if (this.onChanged !== undefined) {
        this.onChanged(exchange);
      }
    };
  }
  public connect() {
    const binanceBookTicker = new MidasSocket('/bookticker/binance');
    binanceBookTicker.addEventListener('message', this.handle('binance'));
  }
}
