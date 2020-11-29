import { Injectable, OnDestroy } from '@angular/core';

import { ISeekInfo } from './iseek-info';
import { MidasSocket } from '../websocket';

export interface IHistChartProg {
  symbol: string;
  num_symbols: number;
  cur_symbol_num: number;
  num_objects: number;
  cur_object_num: number;
}

@Injectable({
  providedIn: 'root'
})
export class HistoricalService implements OnDestroy {
  public readonly syncProgress: Map<string, IHistChartProg>;
  public readonly symbolProgress: ISeekInfo;
  private socket: MidasSocket;

  constructor() {
    this.symbolProgress = {current: 0, size: 0};
    this.socket = new MidasSocket('/historical/subscribe');
    this.socket.addEventListener('message', (ev) => {
      const obj = JSON.parse(ev.data) as IHistChartProg;
      this.syncProgress.set(obj.symbol, obj);
      this.symbolProgress.size = obj.num_symbols;
      if (this.symbolProgress.current < obj.cur_symbol_num) {
        this.symbolProgress.current = obj.cur_symbol_num;
      }
    });
    this.socket.addEventListener('error', (ev) => {
      console.log(ev);
    });
  }

  ngOnDestroy() {
    if (this.socket) {
      this.socket.close();
      delete this.socket;
    }
  }
}
