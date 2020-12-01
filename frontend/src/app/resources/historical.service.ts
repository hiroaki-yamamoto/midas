import { Injectable, OnDestroy } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

import { ISeekInfo } from './iseek-info';
import { MidasSocket } from '../websocket';
import { HistChartProg } from '../rpc/historical_pb'

@Injectable({
  providedIn: 'root'
})
export class HistoricalService implements OnDestroy {
  public readonly syncProgress: Map<string, HistChartProg.AsObject>;
  public readonly symbolProgress: ISeekInfo;
  private socket: MidasSocket;

  constructor(private http: HttpClient) {
    this.symbolProgress = {current: 0, size: 0};
    this.socket = new MidasSocket('/historical/subscribe');
    this.socket.addEventListener('message', (ev) => {
      const obj = JSON.parse(ev.data) as HistChartProg.AsObject;
      this.syncProgress.set(obj.symbol, obj);
      this.symbolProgress.size = obj.numSymbols;
      if (this.symbolProgress.current < obj.curSymbolNum) {
        this.symbolProgress.current = obj.curSymbolNum;
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

  sync(symbols: [string]): Observable<void> {
    return this.http.post<void>('/historical/sync', symbols);
  }
}
