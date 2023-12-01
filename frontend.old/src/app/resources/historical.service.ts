import { Injectable, OnDestroy } from '@angular/core';

import { MidasSocket } from '../websocket';
import { Progress, HistoryFetchRequest } from '../rpc/historical_pb'

@Injectable({
  providedIn: 'root'
})
export class HistoricalService implements OnDestroy {
  public readonly progress: Map<string, Progress.AsObject>;
  private socket: MidasSocket;

  constructor() {
    this.progress = new Map();
    this.socket = new MidasSocket('/historical/subscribe');
    this.socket.addEventListener('message', (ev) => {
      const obj = JSON.parse(ev.data) as Progress.AsObject;
      this.progress.set(obj.symbol, obj);
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

  sync(req: HistoryFetchRequest): void {
    return this.socket.send(JSON.stringify(req.toObject()));
  }

  deleteProgress(symbol: string) {
    this.progress.delete(symbol);
  }
}
