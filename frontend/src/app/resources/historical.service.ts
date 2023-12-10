import { Injectable, OnDestroy } from '@angular/core';

import { MidasSocket } from '../websocket';
import { Progress } from '../../rpc/progress.zod';
import { HistoryFetchRequest } from '../../rpc/history-fetch-request.zod';

@Injectable({
  providedIn: 'root'
})
export class HistoricalService implements OnDestroy {
  public readonly progress: Map<string, Progress>;
  private socket: MidasSocket;

  constructor() {
    this.progress = new Map();
    this.socket = new MidasSocket('/historical/subscribe');
    this.socket.addEventListener('message', (ev) => {
      const obj = Progress.parse(JSON.parse(ev.data));
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
    return this.socket.send(JSON.stringify(req));
  }

  deleteProgress(symbol: string) {
    this.progress.delete(symbol);
  }
}
