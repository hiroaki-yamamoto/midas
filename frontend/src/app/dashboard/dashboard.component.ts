import {
  Component,
  OnInit,
  OnDestroy,
  AfterViewInit,
  ViewChild,
  ElementRef,
} from '@angular/core';

import * as am from '@amcharts/amcharts5';
import * as amxy from '@amcharts/amcharts5/xy';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';
import am5themes_Dark from '@amcharts/amcharts5/themes/Dark';

import { Bot } from '../rpc/bot_pb';
import { BrowserOnlyService } from '../browser-only.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss']
})
export class DashboardComponent implements OnInit, AfterViewInit, OnDestroy {

  public botsInfo: Bot[];
  @ViewChild('graph') private graph: ElementRef;
  private chartRoot: am.Root;

  constructor(private browserOnly: BrowserOnlyService) { }

  ngAfterViewInit() {
    this.browserOnly.browserOnly(() => {
      this.chartRoot = am.Root.new(this.graph.nativeElement);
      const root = this.chartRoot;
      root.setThemes([
        am5themes_Animated.new(root),
        am5themes_Dark.new(root),
      ]);
      let chart = root.container.children.push(
        amxy.XYChart.new(root, {
          layout: root.verticalLayout,
          panY: true,
          panX: true,
          wheelX: 'panX',
          wheelY: 'zoomX',
          cursor: amxy.XYCursor.new(root, {}),
        })
      );
      chart.get('cursor').lineY.set('visible', false);
      const legend = chart.children.push(am.Legend.new(root, {
        x: am.percent(50),
        centerX: am.percent(50),
      }))

      // Create axes
      const dateAxis = chart.xAxes.push(amxy.DateAxis.new(root, {
        renderer: amxy.AxisRendererX.new(root, {}),
        baseInterval: { timeUnit: 'day', count: 1 },
        tooltip: am.Tooltip.new(root, {}),
        tooltipDateFormat: 'yyyy-MM-dd'
      }));

      // Create value axis
      const valueAxis = chart.yAxes.push(amxy.ValueAxis.new(root, {
        renderer: amxy.AxisRendererY.new(root, {}),
      }));

      // Create line series
      const createSeries = (name: string, valueField: string, tooltip: string) => {
        const ret = chart.series.push(amxy.LineSeries.new(root, {
          name,
          valueXField: 'date',
          valueYField: valueField,
          xAxis: dateAxis,
          yAxis: valueAxis,
          tooltip: am.Tooltip.new(root, {labelText: tooltip}),
        }));
        ret.data.processor = am.DataProcessor.new(root, {
          dateFormat: 'yyyy-MM-dd',
          dateFields: ['date'],
        });
        return ret
      };

      const hodl = createSeries(
        'Hodl BTC Profit',
        'hodl',
        'Hodl BTC Profit Ratio: [bold]{valueY}%[/]',
      );

      const bot = createSeries(
        'Bot Trading Profit',
        'bot',
        'Bot Trading Profit Ratio: [bold]{valueY}%[/]'
      );

      const data = [];
      const now = new Date();
      for (let i = 0; i < 365; i++) {
        const time = new Date(now.getTime());
        time.setDate(time.getDate() - i);
        data.push({
          date: time,
          hodl: Math.cos(i / 12) * 100,
          bot: Math.sin(i / 12) * 100,
        });
      }

      hodl.data.setAll(data);
      bot.data.setAll(data);
      legend.data.setAll(chart.series.values);
      hodl.appear(1000);
      bot.appear(1000);
      chart.appear(1000, 100);
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
