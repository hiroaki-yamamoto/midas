import { AfterViewInit, Component, Injectable, OnInit, ViewChild } from '@angular/core';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { HttpClient } from '@angular/common/http';
import { MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';

import { faRotate } from '@fortawesome/free-solid-svg-icons';

import { Exchanges } from '../../rpc/exchanges.zod';
import { SymbolInfo } from '../../rpc/symbol-info.zod';
import { HistoryFetchRequest } from '../../rpc/history-fetch-request.zod';
import { Timestamp } from '../../rpc/timestamp.zod';

import { HistoricalService } from '../resources/historical.service';

@Injectable({ providedIn: 'root' })
class SymbolSyncHandler {
  public symbols: MatTableDataSource<SymbolInfo>;
  public syncButtonEnabled;
  public ignoreSyncBtnReEnable: boolean;

  constructor(public progSock: HistoricalService) {
    this.symbols = new MatTableDataSource<SymbolInfo>();
    this.syncButtonEnabled = true;
    this.ignoreSyncBtnReEnable = false;
  }
  public setPaginator(paginator: MatPaginator) {
    this.symbols.paginator = paginator;
  }
  public next(symbols: { symbols: SymbolInfo[]; }) {
    this.symbols.data = Array.from(new Set(symbols.symbols));
  }
  public error(e) {
    this.symbols.data.length = 0;
    console.error(e);
    this.complete();
  }
  public complete() {
    if (!this.ignoreSyncBtnReEnable) {
      setTimeout(() => { this.syncButtonEnabled = true; }, 3000);
    }
    this.ignoreSyncBtnReEnable = false;
  }
}

@Component({
  standalone: false,
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent implements OnInit, AfterViewInit {

  public exchange: Exchanges;
  public rotateIcon = faRotate;
  public dispCol = ['symbol', 'syncBtns'];
  @ViewChild(MatPaginator) symbolPaginator: MatPaginator;

  constructor(
    private curRoute: ActivatedRoute,
    private http: HttpClient,
    public syncHandler: SymbolSyncHandler,
  ) {
  }

  ngOnInit(): void {
    this.curRoute.paramMap.subscribe((params: ParamMap) => {
      this.exchange = Exchanges.parse(params.get('exchange'));
      this.syncHandler.ignoreSyncBtnReEnable = true;
      this.http.get(`/symbol/currencies/${this.exchange}`)
        .subscribe(this.syncHandler);
    });
  }

  ngAfterViewInit(): void {
    this.syncHandler.setPaginator(this.symbolPaginator);
  }

  public isDisabledAll(): boolean {
    const symbols = new Set(
      this.syncHandler.symbols.data.map((value) => { return value.symbol; })
    );
    this.syncHandler.progSock.progress.forEach((_, symbol) => { symbols.delete(symbol); });
    return symbols.size === 0;
  }

  public syncAll(): void {
    this.syncHandler.symbols.data.forEach((value) => {
      if (!this.syncHandler.progSock.progress.has(value.symbol)) {
        this.syncHandler.progSock.progress.set(value.symbol, null);
      }
    });
  }

  public sync(symbol: string): void {
    const now = new Date();
    this.syncHandler.progSock.progress.set(symbol, null);
    const req = HistoryFetchRequest.parse({
      exchange: this.exchange,
      symbol,
      start: Timestamp.parse({ nanos: 0, secs: 0 }),
      end: Timestamp.parse({
        secs: parseInt((now.getTime() / 1000).toString(), 10),
        nanos: parseInt((now.getMilliseconds() * 1000000).toString(), 10),
      }),
    });
    this.syncHandler.progSock.sync(req);
  }

  public syncSymbol(): void {
    this.syncHandler.syncButtonEnabled = false;
    this.http.post(`/symbol/refresh/${this.exchange}`, undefined)
      .subscribe(this.syncHandler);
  }

  public find(event: Event) {
    const text = (event.target as HTMLInputElement).value;
    this.syncHandler.symbols.filter = text.trim().toLowerCase();
  }
}
