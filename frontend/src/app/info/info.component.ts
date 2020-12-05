import { Component, OnInit, AfterViewInit, ViewChild } from '@angular/core';
import { MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';

import { Exchanges } from '../rpc/entities_pb';
import { BookTicker } from '../rpc/bookticker_pb';
import { TradeObserverService } from '../resources/trade-observer.service';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatPaginatedTabHeader } from '@angular/material/tabs/paginated-tab-header';

@Component({
  selector: 'app-info',
  templateUrl: './info.component.html',
  styleUrls: ['./info.component.scss']
})
export class InfoComponent implements OnInit, AfterViewInit {
  public exchanges: string[];
  public columnsToDisplay = [
    'symbol', 'askPrice', 'askQty', 'bidPrice', 'bidQty'
  ];
  public dataSource: { [key: string]: MatTableDataSource<BookTicker> };
  @ViewChild(MatPaginatedTabHeader) public paginator: MatPaginator;

  constructor(private booktickerObserver: TradeObserverService) {
    this.exchanges = Object.keys(Exchanges).map((upperName) => {
      return upperName[0].toUpperCase() + upperName.substr(1).toLowerCase();
    });
    this.exchanges.forEach((exchangeName) => {
      this.dataSource[exchangeName] = new MatTableDataSource(Object.values(
        this.booktickerObserver[exchangeName.toLowerCase()]
      ));
    });
  }

  ngOnInit(): void {
  }

  ngAfterViewInit(): void {

  }
}
