import { Component, OnInit, OnDestroy } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import {
  faTimes,
  faSyncAlt,
  faHistory,
  faSkullCrossbones,
} from '@fortawesome/free-solid-svg-icons';

import { Exchanges } from '../resources/exchanges.enum';
import { Exchanges as RPCExchanges } from '../rpc/entities_pb';
import { HistChartFetchReq } from '../rpc/historical_pb';

import { IHistChartProg } from '../sync-progress/entities';
import { ISymbol } from './entities'
import { SymbolService } from '../resources/symbol.service';

import {
  IconSnackBarComponent
} from '../icon-snackbar/icon-snackbar.component';
import { MidasSocket } from '../websocket';

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
  progList: Map<string, IHistChartProg>;
  symbol: ISymbol;

  private histClient;
  private histStreamClient: WebSocket;

  constructor(private tooltip: MatSnackBar, private symbolClient: SymbolService) {
    this.progList = new Map();
    this.symbol = { num: 0, cur: 0 };
  }

  ngOnInit(): void {
    this.histClient = undefined;
    this.histStreamClient = new MidasSocket('/historical/subscribe');
    this.histStreamClient.addEventListener('message', (ev) => {
      const obj = JSON.parse(ev.data) as IHistChartProg;
      this.progList.set(obj.symbol, obj);
      this.symbol.num = obj.num_symbols;
      if (this.symbol.cur < obj.cur_symbol_num) {
        this.symbol.cur = obj.cur_symbol_num;
      }
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
    this.symbolClient.refresh<void>(RPCExchanges.BINANCE).subscribe(
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
      },
      () => { this.symbolButtonEnabled = true; }
    );
  }

  fetchHistoricalData() {
    const req = new HistChartFetchReq();
    req.setExchange(RPCExchanges.BINANCE);
    req.setSymbolsList(['all']);
    this.histClient.sync(req, {}, () => {});
  }

  fetchProgressCompleted(ev: IHistChartProg) {
    this.progList.delete(ev.symbol);
  }
}
