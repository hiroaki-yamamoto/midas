import {
  Component,
  Input,
  OnInit,
  ViewChild,
  ElementRef,
  NgZone,
} from '@angular/core';

import { MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';

import { create, color } from '@amcharts/amcharts4/core';
import {
  XYChart,
  XYCursor,
  LineSeries,
  DateAxis,
  ValueAxis,
  Legend,
} from '@amcharts/amcharts4/charts';

import { Bot, Position } from '../rpc/bot_pb';
import { IGraphStats } from './interfaces';

@Component({
  selector: 'app-bot-panel',
  templateUrl: './bot-panel.component.html',
  styleUrls: ['./bot-panel.component.scss']
})
export class BotPanelComponent implements OnInit {

  @Input() bot: Bot;
  @ViewChild('profitGraph') profitGraph: ElementRef;
  @ViewChild('curPosPaginator', {static: true}) curPosPaginator: MatPaginator;
  @ViewChild('arcPosPaginator', { static: true }) arcPosPaginator: MatPaginator;

  private g: XYChart;

  public currentPositions: MatTableDataSource<Position>;
  public archivedPositions: MatTableDataSource<Position>;
  public objItems = Object.entries;
  public dispCol: string[] = [
    'symbol', 'tradingAmount', 'valuation', 'profitAmount', 'profitPercent',
  ];

  constructor(private zone: NgZone) {
    this.currentPositions = new MatTableDataSource<Position>([]);
    this.archivedPositions = new MatTableDataSource<Position>([]);
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
      pos.setTradingamount(Math.random());
      pos.setValuation(
        pos.getTradingamount() + (
          ((Math.round(Math.random() * 10) & 0x01) ? 1 : - 1) *
          Math.random()
        )
      );
      this.currentPositions.data = this.currentPositions.data.concat(pos);
    }

    for (let i = 0; i < 20; i++) {
      const id = `test-cur-position-${i}`;
      const pos = new Position();
      pos.setId(id);
      pos.setBotid(this.bot.getId());
      pos.setSymbol('TESTUSDT');
      pos.setTradingamount(Math.random());
      pos.setValuation(
        pos.getTradingamount() + (
          ((Math.round(Math.random() * 10) & 0x01) ? 1 : - 1) *
          Math.random()
        )
      );
      this.archivedPositions.data = this.archivedPositions.data.concat(pos);
    }

    const botStats: IGraphStats[] = [];
    const txtColor = color('#ffffff');

    for (let i = 0; i < 90; i++) {
      const time = new Date();
      time.setDate(time.getDate() - i);
      botStats.push({
        date: time,
        realizedProfitPercent: Math.sin(i / 12) * 100,
        unrealizedProfitPercent: Math.sin(i / 24) * 100,
      });
    }
    this.zone.runOutsideAngular(() => {
      this.g = create(this.profitGraph.nativeElement, XYChart);

      this.g.cursor = new XYCursor();
      this.g.cursor.lineY.opacity = 0;
      this.g.legend = new Legend();
      this.g.legend.labels.template.fill = txtColor;

      // Create axes
      const dateAxis = this.g.xAxes.push(new DateAxis());
      dateAxis.renderer.minGridDistance = 50;
      dateAxis.renderer.grid.template.location = 0.5;
      dateAxis.startLocation = 0.5;
      dateAxis.endLocation = 0.5;
      dateAxis.renderer.grid.template.stroke = txtColor;
      dateAxis.renderer.labels.template.fill = txtColor;
      dateAxis.tooltipDateFormat = 'd MMMM';

      // Create value axis
      const valueAxis = this.g.yAxes.push(new ValueAxis());
      valueAxis.renderer.grid.template.stroke = txtColor;
      valueAxis.renderer.labels.template.fill = txtColor;

      // Create realized profitability series.
      const bot = this.g.series.push(new LineSeries());
      bot.dataFields.dateX = 'date';
      bot.dataFields.valueY = 'realizedProfitPercent';
      bot.strokeWidth = 1;
      bot.tensionX = 1.0;
      bot.fillOpacity = 0.5;
      bot.name = 'Realized Profit Percent';
      bot.tooltipText =
        'Realized Trading Profit Ratio: [bold]{realizedProfitPercent}%[/]';

      // Create unrealized profitability series.
      const uProfitPercent = this.g.series.push(new LineSeries());
      uProfitPercent.dataFields.dateX = 'date';
      uProfitPercent.dataFields.valueY = 'unrealizedProfitPercent';
      uProfitPercent.strokeWidth = 1;
      uProfitPercent.tensionX = 1.0;
      uProfitPercent.fillOpacity = 0.5;
      uProfitPercent.name = 'Realized Profit Percent';
      uProfitPercent.tooltipText =
        'Realized Trading Profit Ratio: [bold]{unrealizedProfitPercent}%[/]';

      // Set the data.
      this.g.data = botStats;
    });
  }

  close() {
    this.zone.runOutsideAngular(() => {
      if (this.g) {
        this.g.dispose();
      }
    });
  }
}
