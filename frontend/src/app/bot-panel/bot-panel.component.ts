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

import * as am from '@amcharts/amcharts5';
import * as amxy from '@amcharts/amcharts5/xy';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';

import { Bot, Position } from '../rpc/bot_pb';
import { IGraphStats } from './interfaces';
import { BrowserOnlyService } from '../browser-only.service';

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

  private chartRoot: am.Root;

  public currentPositions: MatTableDataSource<Position>;
  public archivedPositions: MatTableDataSource<Position>;
  public objItems = Object.entries;
  public dispCol: string[] = [
    'symbol', 'tradingAmount', 'valuation', 'profitAmount', 'profitPercent',
  ];

  constructor(private browserOnly: BrowserOnlyService) {
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
      pos.setTradingAmount(Math.random());
      pos.setValuation(
        pos.getTradingAmount() + (
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
      pos.setTradingAmount(Math.random());
      pos.setValuation(
        pos.getTradingAmount() + (
          ((Math.round(Math.random() * 10) & 0x01) ? 1 : - 1) *
          Math.random()
        )
      );
      this.archivedPositions.data = this.archivedPositions.data.concat(pos);
    }

    const botStats: IGraphStats[] = [];

    for (let i = 0; i < 90; i++) {
      const time = new Date();
      time.setDate(time.getDate() - i);
      botStats.push({
        date: time,
        realizedProfitPercent: Math.sin(i / 12) * 100,
        unrealizedProfitPercent: Math.sin(i / 24) * 100,
      });
    }
    this.browserOnly.browserOnly(() => {
      let root = am.Root.new(this.profitGraph.nativeElement);
      root.setThemes([am5themes_Animated.new(root)]);
      let chart = root.container.children.push(
        amxy.XYChart.new(root, { panY: true, layout: root.verticalLayout })
      );

      // Create axes
      const dateAxis = chart.xAxes.push(amxy.DateAxis.new(root, {
        renderer: amxy.AxisRendererX.new(root, { pan: 'zoom' }),
        baseInterval: { timeUnit: 'day', count: 1 },
        tooltip: am.Tooltip.new(root, {
          dateFormatter: am.DateFormatter.new(root, { dateFormat: 'd MMMM' })
        }),
        gridIntervals: [{ timeUnit: 'day', count: 1 }],
        maxDeviation: 0.5,
      }));

      // Create value axis
      const valueAxis = chart.yAxes.push(amxy.ValueAxis.new(root, {
        renderer: amxy.AxisRendererY.new(root, {})
      }));

      // Create realized profitability series.
      const realized = chart.series.push(amxy.LineSeries.new(root, {
        name: 'Realized Profit Percent',
        valueXField: 'date',
        valueYField: 'realizedProfitPercent',
        calculateAggregates: true,
        xAxis: dateAxis,
        yAxis: valueAxis,
        legendValueText: '{valueY}',
        tooltip: am.Tooltip.new(root, {
          pointerOrientation: 'horizontal',
          labelText: `Realized Trading Profit Ratio: \
                    [bold]{realizedProfitPercent}%[/]`,
        }),
      }));

      // Create unrealized profitability series.
      const unRealized = chart.series.push(amxy.LineSeries.new(root, {
        name: 'Un-Realized Profit Percent',
        valueXField: 'date',
        valueYField: 'unrealizedProfitPercent',
        calculateAggregates: true,
        xAxis: dateAxis,
        yAxis: valueAxis,
        legendValueText: '{valueY}',
        tooltip: am.Tooltip.new(root, {
          pointerOrientation: 'horizontal',
          labelText: `Un-Realized Trading Profit Ratio: \
                    [bold]{unrealizedProfitPercent}%[/]`,
        }),
      }));
      realized.data.setAll(botStats);
      unRealized.data.setAll(botStats);

      // Set the data.
      this.chartRoot = root;
    });
  }

  close() {
    this.browserOnly.browserOnly(() => {
      if (this.chartRoot) {
        this.chartRoot.dispose();
      }
    });
  }
}
