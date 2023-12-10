import { Component, Input, ViewChild, AfterViewInit } from '@angular/core';
import { MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';
import { MatSort } from '@angular/material/sort'

import { BookTicker } from '../../rpc/bookticker_pb';
import { TradeObserverService } from '../../resources/trade-observer.service';

@Component({
  selector: 'app-book-ticker',
  templateUrl: './book-ticker.component.html',
  styleUrls: ['./book-ticker.component.scss']
})
export class BookTickerComponent implements AfterViewInit {
  @Input() public exchange: string;
  @ViewChild(MatPaginator) public paginator: MatPaginator;
  @ViewChild(MatSort) public sort: MatSort;
  public dataSource: MatTableDataSource<BookTicker>;
  public columnsToDisplay = [
    'symbol', 'askPrice', 'askQty', 'bidPrice', 'bidQty'
  ];

  constructor(private booktickerObserver: TradeObserverService) {
    this.dataSource = new MatTableDataSource();
    this.booktickerObserver.onChanged = (/*exchange: string*/) => {
      this.dataSource.data = Object.values(
        this.booktickerObserver[this.exchange.toLowerCase()]
      );
    };
  }

  ngAfterViewInit(): void {
    this.dataSource.sort = this.sort;
    this.dataSource.paginator = this.paginator;
  }
}
