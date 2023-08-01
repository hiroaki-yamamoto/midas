import {
  Component,
  Input,
  OnInit,
  ViewChild,
} from '@angular/core';

import { MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';

import { Bot, Position } from '../rpc/bot_pb';
import { IGraphStats } from './interfaces';
import { BrowserOnlyService } from '../browser-only.service';
import { ISeries } from '../date-graph/date-graph.component';

@Component({
  selector: 'app-bot-panel',
  templateUrl: './bot-panel.component.html',
  styleUrls: ['./bot-panel.component.scss']
})
export class BotPanelComponent implements OnInit {

  @Input() bot: Bot;
  @ViewChild('curPosPaginator', { static: true }) curPosPaginator: MatPaginator;
  @ViewChild('arcPosPaginator', { static: true }) arcPosPaginator: MatPaginator;

  public currentPositions: MatTableDataSource<Position>;
  public archivedPositions: MatTableDataSource<Position>;
  public objItems = Object.entries;
  public dispCol: string[] = [
    'symbol', 'tradingAmount', 'valuation', 'profitAmount', 'profitPercent',
  ];
  public data: IGraphStats[];
  public series: ISeries[] = [
    {
      name: 'Realized Profit Percent',
      valueField: 'realizedProfitPercent',
      tooltip: `Realized Trading Profit Ratio: \
                [bold]{realizedProfitPercent}%[/]`,
    },
    {
      name: 'Un-Realized Profit Percent',
      valueField: 'unrealizedProfitPercent',
      tooltip: `Un-Realized Trading Profit Ratio: \
                [bold]{unrealizedProfitPercent}%[/]`,
    }
  ];

  constructor(private browserOnly: BrowserOnlyService) {
    this.currentPositions = new MatTableDataSource<Position>([]);
    this.archivedPositions = new MatTableDataSource<Position>([]);

    this.data = [];
    const time = new Date();
    for (let i = 0; i < 90; i++) {
      const clonedTime = new Date(time.getTime());
      clonedTime.setDate(time.getDate() - i);
      this.data.push({
        date: clonedTime,
        realizedProfitPercent: Math.sin(i / 12) * 100,
        unrealizedProfitPercent: Math.sin(i / 24) * 100,
      });
    }
  }

  ngOnInit() {
    this.currentPositions.paginator = this.curPosPaginator;
    this.archivedPositions.paginator = this.arcPosPaginator;
  }

  open() {
    for (let i = 0; i < 20; i++) {
      const id = `test-cur-position-${i}`;
      const pos = new Position();
      pos.setId(id);
      pos.setBotid(this.bot.getId());
      pos.setSymbol('TESTUSDT');
      pos.setTradingAmount(Math.random().toString());
      pos.setValuation(
        (parseFloat(pos.getTradingAmount()) + (
          ((Math.round(Math.random() * 10) & 0x01) ? 1 : - 1) *
          Math.random()
        )).toString()
      );
      this.currentPositions.data = this.currentPositions.data.concat(pos);
    }

    for (let i = 0; i < 20; i++) {
      const id = `test-cur-position-${i}`;
      const pos = new Position();
      pos.setId(id);
      pos.setBotid(this.bot.getId());
      pos.setSymbol('TESTUSDT');
      pos.setTradingAmount(Math.random().toString());
      pos.setValuation(
        (parseFloat(pos.getTradingAmount()) + (
          ((Math.round(Math.random() * 10) & 0x01) ? 1 : - 1) *
          Math.random()
        )).toString()
      );
      this.archivedPositions.data = this.archivedPositions.data.concat(pos);
    }
  }
}
