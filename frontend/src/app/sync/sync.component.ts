import { Component, OnInit, NgZone, OnDestroy } from '@angular/core';
import { faTimes, faSyncAlt, faHistory } from '@fortawesome/free-solid-svg-icons';

import { HistChartClient } from '../rpc/historical_grpc_web_pb';
import { HistChartProg } from '../rpc/historical_pb';
import { Empty } from 'google-protobuf/google/protobuf/empty_pb';
import { ClientReadableStream } from 'grpc-web';

@Component({
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent implements OnInit, OnDestroy {
  closeIcon = faTimes;
  syncIcon = faSyncAlt;
  histIcon = faHistory;

  private client: HistChartClient;
  private subscribeStream: ClientReadableStream<HistChartProg>;

  constructor(private zone: NgZone) { }

  ngOnInit(): void {
    this.zone.runOutsideAngular(() => {
      this.client = new HistChartClient('/historical', );
      this.subscribeStream = this.client.subscribe(new Empty(), {});
      this.subscribeStream.on('data', (resp) => {
      });
    })
  }

  ngOnDestroy(): void {
    this.zone.runOutsideAngular(() => {
      this.subscribeStream.cancel;
      this.subscribeStream = undefined;
    });
  }
}
