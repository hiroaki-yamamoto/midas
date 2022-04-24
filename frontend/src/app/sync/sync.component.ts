import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { HttpClient } from '@angular/common/http';

import { faRotate } from '@fortawesome/free-solid-svg-icons';

import { Exchanges } from '../rpc/entities_pb';
import { SymbolList } from '../rpc/symbols_pb';

class SymbolSyncHandler {
  public symbols: string[] = [];
  public syncButtonEnabled: boolean = true
  public next(symbols: SymbolList.AsObject) {
    this.symbols = symbols.symbolsList;
  }
  public error(e) {
    console.error(e);
    this.complete();
  }
  public complete() {
    setTimeout(() => { this.syncButtonEnabled = true }, 3000);
  }
}

@Component({
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent implements OnInit {

  public exchange: Exchanges;
  public rotateIcon = faRotate;
  public syncHandler = new SymbolSyncHandler();

  constructor(private curRoute: ActivatedRoute, private http: HttpClient) {}

  ngOnInit(): void {
    this.curRoute.paramMap.subscribe((params: ParamMap) => {
      this.exchange = parseInt(params.get('exchange'), 10) as Exchanges;
    });
  }

  public syncSymbol(): void {
    this.syncHandler.syncButtonEnabled = false;
    this.http.post(`/symbol/refresha/${this.exchange}`, undefined)
      .subscribe(this.syncHandler);
  }

}
