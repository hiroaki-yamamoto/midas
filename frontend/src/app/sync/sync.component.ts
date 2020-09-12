import { Component, OnInit, NgZone, OnDestroy } from '@angular/core';
import { faTimes, faSyncAlt, faHistory } from '@fortawesome/free-solid-svg-icons';

import { Empty } from 'google-protobuf/google/protobuf/empty_pb';
import { ClientReadableStream } from 'grpc-web';

import { Exchanges } from '../rpc/entities_pb';
import { HistChartClient } from '../rpc/historical_grpc_web_pb';
import { HistChartProg } from '../rpc/historical_pb';
import { SymbolPromiseClient } from '../rpc/symbol_grpc_web_pb';
import { RefreshRequest as SymbolRefreshRequest } from '../rpc/symbol_pb';

@Component({
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent implements OnInit, OnDestroy {
  closeIcon = faTimes;
  syncIcon = faSyncAlt;
  histIcon = faHistory;

  private histClient: HistChartClient;
  private histStreamClient: WebSocket;

  private symbolClient: SymbolPromiseClient;

  constructor(private zone: NgZone) { }

  ngOnInit(): void {
    this.zone.runOutsideAngular(() => {
      this.histClient = new HistChartClient('historical', null, null);
      this.symbolClient = new SymbolPromiseClient('symbol', null, null);
      let loc = window.location;
      const streamURI =
        `${((loc.protocol === 'https:') ? 'wss' : 'ws')}://${loc.host}/historical/stream/subscribe`;
      this.histStreamClient = new WebSocket(streamURI);
      this.histStreamClient.addEventListener('error', (ev) => {
        console.log(ev);
      });
    })
  }

  ngOnDestroy(): void {
    this.zone.runOutsideAngular(() => {
      this.histStreamClient.close();
      this.histStreamClient = undefined;
    });
  }

  fetchSymbol() {
    const req = new SymbolRefreshRequest();
    req.setExchange(Exchanges.BINANCE);
    this.symbolClient.refresh(req).then(
      () => {console.log('Done')},
      (e) => { console.error(e) }
    );
  }
}
