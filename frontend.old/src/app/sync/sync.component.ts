import { AfterViewInit, Component, Injectable, OnInit, ViewChild } from '@angular/core';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { HttpClient } from '@angular/common/http';
import { MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';

import { faRotate } from '@fortawesome/free-solid-svg-icons';

import { Exchanges } from '../rpc/entities_pb';
import { SymbolInfo } from '../rpc/symbols_pb';
import { HistoryFetchRequest } from '../rpc/historical_pb';
import { Timestamp } from 'google-protobuf/google/protobuf/timestamp_pb';

import { HistoricalService } from '../resources/historical.service';

@Injectable({ providedIn: 'root' })
class SymbolSyncHandler {
  public symbols: MatTableDataSource<SymbolInfo.AsObject>;
  public syncButtonEnabled;
  public ignoreSyncBtnReEnable: boolean;

  constructor(public progSock: HistoricalService) {
    this.symbols = new MatTableDataSource<SymbolInfo.AsObject>();
    this.syncButtonEnabled = true;
    this.ignoreSyncBtnReEnable = false;
  }
  public setPaginator(paginator: MatPaginator) {
    this.symbols.paginator = paginator;
  }
  public next(symbols: { symbols: SymbolInfo.AsObject[] }) {
    this.symbols.data = Array.from(new Set(symbols.symbols));
  }
  public error(e) {
    this.symbols.data.length = 0;
    console.error(e);
    this.complete();
  }
  public complete() {
    if (!this.ignoreSyncBtnReEnable) {
      setTimeout(() => { this.syncButtonEnabled = true }, 3000);
    }
    this.ignoreSyncBtnReEnable = false;
  }
}

@Component({
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
      this.exchange = parseInt(params.get('exchange'), 10) as Exchanges;
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
    this.syncHandler.progSock.progress.set(symbol, null);
    const req = new HistoryFetchRequest();
    req.setExchange(this.exchange);
    req.setSymbol(symbol);
    req.setStart(new Timestamp());
    req.setEnd(Timestamp.fromDate(new Date()));
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