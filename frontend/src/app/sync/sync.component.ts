import { Component, OnInit, OnDestroy } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import { faTimes, faSyncAlt, faHistory } from '@fortawesome/free-solid-svg-icons';

import { Empty } from 'google-protobuf/google/protobuf/empty_pb';

import { Exchanges } from '../rpc/entities_pb';
import { HistChartClient } from '../rpc/historical_grpc_web_pb';
import { HistChartProg } from '../rpc/historical_pb';
import { SymbolPromiseClient } from '../rpc/symbol_grpc_web_pb';
import { RefreshRequest as SymbolRefreshRequest } from '../rpc/symbol_pb';

import {
  IconSnackBarComponent,
  NotificationLevel
} from '../icon-snackbar/icon-snackbar.component';
import { MidasWebSocket } from '../websocket';

@Component({
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent implements OnInit, OnDestroy {
  closeIcon = faTimes;
  syncIcon = faSyncAlt;
  histIcon = faHistory;
  symbolButtonEnabled = true;

  private histClient: HistChartClient;
  private histStreamClient: WebSocket;

  private symbolClient: SymbolPromiseClient;

  constructor(private tooltip: MatSnackBar) { }

  ngOnInit(): void {
    this.histClient = new HistChartClient('historical', null, null);
    this.symbolClient = new SymbolPromiseClient('symbol', null, null);
    this.histStreamClient = new MidasWebSocket('/historical/stream/subscribe');
    this.histStreamClient.addEventListener('error', (ev) => {
      console.log(ev);
    });
  }

  ngOnDestroy(): void {
    this.histStreamClient.close();
    this.histStreamClient = undefined;
  }

  fetchSymbol() {
    this.symbolButtonEnabled = false;
    const req = new SymbolRefreshRequest();
    req.setExchange(Exchanges.BINANCE);
    this.symbolClient.refresh(req).then(
      () => {},
      (e) => {
        this.tooltip.openFromComponent(IconSnackBarComponent, {
          data: {
            level: NotificationLevel.Error,
            message: e.message,
          },
        });
        console.error(e);
      }
    ).finally(() => {this.symbolButtonEnabled = true;});
  }
}
