import {
  Component,
  OnInit,
  OnDestroy,
  AfterViewInit,
} from '@angular/core';

import * as am from '@amcharts/amcharts5';
import * as amxy from '@amcharts/amcharts5/xy';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';

import {Bot, TriggerType} from '../rpc/bot_pb';
import { BrowserOnlyService } from '../browser-only.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss']
})
export class DashboardComponent implements OnInit, AfterViewInit, OnDestroy {

  public botsInfo: Bot[];
  private chartRoot: am.Root;

  constructor(private browserOnly: BrowserOnlyService) { }

  ngAfterViewInit() {
    this.browserOnly.browserOnly(() => {
      let root = am.Root.new('compare-graph');
      root.setThemes([am5themes_Animated.new(root)]);
      let chart = root.container.children.push(
        amxy.XYChart.new(root, {panY: true, layout: root.verticalLayout})
      );
      const data = [];
      for (let i = 0; i < 365; i++) {
        const time = new Date();
        time.setDate(time.getDate() - i);
        data.push({
          date: time,
          hodl: Math.cos(i / 12) * 100,
          bot: Math.sin(i / 12) * 100,
        });
      }

      chart.set('cursor', amxy.XYCursor.new(root, {}));

      // Create axes
      const dateAxis = chart.xAxes.push(amxy.DateAxis.new(root, {
        renderer: amxy.AxisRendererX.new(root, {pan: 'zoom'}),
        baseInterval: { timeUnit: 'day', count: 1 },
        tooltip: am.Tooltip.new(root, {
          dateFormatter: am.DateFormatter.new(root, { dateFormat: 'd MMMM'})
        }),
        maxDeviation: 0.1,
      }));

      // Create value axis
      const valueAxis = chart.yAxes.push(amxy.ValueAxis.new(root, {
        maxDeviation: 1,
        renderer: amxy.AxisRendererY.new(root, {})
      }));

      const hodl = chart.series.push(amxy.LineSeries.new(root, {
        name: 'Hodl BTC Profit',
        valueXField: 'date',
        valueYField: 'hodl',
        calculateAggregates: true,
        xAxis: dateAxis,
        yAxis: valueAxis,
        legendValueText: '{valueY}',
        tooltip: am.Tooltip.new(root, {
          pointerOrientation: 'horizontal',
          labelText: 'Hodl BTC Profit Ratio: [bold]{hodl}%[/]',
        }),
      }));
      hodl.data.processor = am.DataProcessor.new(root, {
        dateFormat: 'yyyy-MM-dd',
        dateFields: ['date'],
      });

      const bot = chart.series.push(amxy.LineSeries.new(root, {
        name: 'Bot Trading Profit',
        valueXField: 'date',
        valueYField: 'bot',
        calculateAggregates: true,
        xAxis: dateAxis,
        yAxis: valueAxis,
        legendValueText: '{valueY}',
        tooltip: am.Tooltip.new(root, {
          pointerOrientation: 'horizontal',
          labelText: 'Bot Trading Profit Ratio: [bold]{bot}%[/]',
        }),
      }));
      bot.data.processor = am.DataProcessor.new(root, {
        dateFormat: 'yyyy-MM-dd',
        dateFields: ['date'],
      });

      hodl.data.setAll(data);
      bot.data.setAll(data);

      this.chartRoot = root;
    });
  }

  ngOnInit(): void {
    this.botsInfo = [];
    for (let i = 0; i < 5; i++) {
      const info = new Bot();
      info.setId(`test-bot-${i}`);
      info.setName(`Test Bot ${i}`);
      this.botsInfo = this.botsInfo.concat(info);
    }
  }

  ngOnDestroy() {
    this.browserOnly.browserOnly(() => {
      if (this.chartRoot) {
        this.chartRoot.dispose()
      }
    });
  }
}
