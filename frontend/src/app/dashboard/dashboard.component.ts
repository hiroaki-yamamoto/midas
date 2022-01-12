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

import {Bot, TriggerType} from '../rpc/bot_pb';
import { BrowserOnlyService } from '../browser-only.service';
import { leadingComment } from '@angular/compiler';

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
      let root = am.Root.new(this.graph.nativeElement);
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
        })
      );
      const legend = chart.children.push(am.Legend.new(root, {
        x: am.percent(50),
        centerX: am.percent(50),
      }))
      // Cursor
      chart.set('cursor', amxy.XYCursor.new(root, {}));


      // Create axes
      const dateAxis = chart.xAxes.push(amxy.DateAxis.new(root, {
        maxDeviation: 0.5,
        renderer: amxy.AxisRendererX.new(root, {}),
        baseInterval: { timeUnit: 'day', count: 1 },
        tooltip: am.Tooltip.new(root, {}),
        tooltipDateFormat: 'yyyy-MM-dd'
      }));

      // Create value axis
      const valueAxis = chart.yAxes.push(amxy.ValueAxis.new(root, {
        maxDeviation: 1,
        renderer: amxy.AxisRendererY.new(root, {}),
        tooltip: am.Tooltip.new(root, {}),
      }));

      const hodl = chart.series.push(amxy.LineSeries.new(root, {
        name: 'Hodl BTC Profit',
        valueXField: 'date',
        valueYField: 'hodl',
        xAxis: dateAxis,
        yAxis: valueAxis,
        tooltip: am.Tooltip.new(root, {}),
        tooltipText: 'Hodl BTC Profit Ratio: [bold]{valueY}%[/]',
      }));
      hodl.data.processor = am.DataProcessor.new(root, {
        dateFormat: 'yyyy-MM-dd',
        dateFields: ['date'],
      });

      const bot = chart.series.push(amxy.LineSeries.new(root, {
        name: 'Bot Trading Profit',
        valueXField: 'date',
        valueYField: 'bot',
        xAxis: dateAxis,
        yAxis: valueAxis,
        tooltip: am.Tooltip.new(root, {}),
        tooltipText: 'Bot Trading Profit Ratio: [bold]{valueY}%[/]',
      }));
      bot.data.processor = am.DataProcessor.new(root, {
        dateFormat: 'yyyy-MM-dd',
        dateFields: ['date'],
      });

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
      dateAxis.data.setAll(data);
      hodl.appear(1000);
      bot.appear(1000);
      chart.appear(1000, 100);


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
