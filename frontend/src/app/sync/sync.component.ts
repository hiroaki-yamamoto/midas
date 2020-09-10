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
  private subscribeStream: ClientReadableStream<HistChartProg>;

  private symbolClient: SymbolPromiseClient;

  constructor(private zone: NgZone) { }

  ngOnInit(): void {
    this.zone.runOutsideAngular(() => {
      this.histClient = new HistChartClient('historical', null, null);
      this.subscribeStream = this.histClient.subscribe(new Empty(), {});
      this.subscribeStream.on('data', (resp) => {
      });

      this.symbolClient = new SymbolPromiseClient('symbol', null, null);
    })
  }

  ngOnDestroy(): void {
    this.zone.runOutsideAngular(() => {
      this.subscribeStream.cancel;
      this.subscribeStream = undefined;
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
