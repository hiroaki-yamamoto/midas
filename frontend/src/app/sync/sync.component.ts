import { Component, OnInit, OnDestroy } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import {
  faTimes,
  faSyncAlt,
  faHistory,
  faSkullCrossbones,
} from '@fortawesome/free-solid-svg-icons';

import { Exchanges } from '../rpc/entities_pb';
import { HistChartFetchReq } from '../rpc/historical_pb';
import { HistChartClient } from '../rpc/historical_grpc_web_pb';
import { SymbolPromiseClient } from '../rpc/symbol_grpc_web_pb';
import { RefreshRequest as SymbolRefreshRequest } from '../rpc/symbol_pb';

import {
  IconSnackBarComponent
} from '../icon-snackbar/icon-snackbar.component';
import { MidasSocket } from '../websocket';

interface HistChartProg {
  symbol: string;
  num_symbols: number;
  cur_symbol_num: number;
  num_objects: number;
  cur_object_num: number;
}

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
  progList: Map<string, HistChartProg> = undefined;

  private histClient: HistChartClient;
  private histStreamClient: WebSocket;

  private symbolClient: SymbolPromiseClient;

  constructor(private tooltip: MatSnackBar) {
    this.progList = new Map();
  }

  ngOnInit(): void {
    this.histClient = new HistChartClient('historical', null, null);
    this.symbolClient = new SymbolPromiseClient('symbol', null, null);
    this.histStreamClient = new MidasSocket('/historical/stream/subscribe');
    this.histStreamClient.addEventListener('message', (ev) => {
      const obj = JSON.parse(ev.data) as HistChartProg;
      this.progList = this.progList.set(obj.symbol, obj);
    });
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
            icon: faSkullCrossbones,
            actionTxt: 'Dismiss',
            message: e.message,
          },
        });
        console.error(e);
      }
    ).finally(() => {this.symbolButtonEnabled = true;});
  }

  fetchHistoricalData() {
    const req = new HistChartFetchReq();
    req.setExchange(Exchanges.BINANCE);
    req.setSymbolsList(['all']);
    this.histClient.sync(req, {}, () => {});
  }
}
