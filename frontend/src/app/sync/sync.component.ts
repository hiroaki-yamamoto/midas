import { Component } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import {
  faTimes,
  faSyncAlt,
  faHistory,
  faSkullCrossbones,
} from '@fortawesome/free-solid-svg-icons';

import { Exchanges as RPCExchanges } from '../rpc/entities_pb';
import { HistChartProg, HistChartFetchReq } from '../rpc/historical_pb';

import { HistoricalService } from '../resources/historical.service';
import { SymbolService } from '../resources/symbol.service';

import {
  IconSnackBarComponent
} from '../icon-snackbar/icon-snackbar.component';

@Component({
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent {
  closeIcon = faTimes;
  syncIcon = faSyncAlt;
  histIcon = faHistory;
  symbolButtonEnabled = true;

  constructor(
    private tooltip: MatSnackBar,
    private symbolClient: SymbolService,
    public historicalClient: HistoricalService,
  ) {
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
    req.setSymbolsList(['all']);
    this.historicalClient.sync(RPCExchanges.BINANCE, req).subscribe();
  }

  fetchProgressCompleted(ev: HistChartProg.AsObject) {
    this.historicalClient.deleteProgress(ev.symbol);
  }
}
