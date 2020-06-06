import {
  Component,
  OnInit,
  NgZone,
  AfterViewInit,
} from '@angular/core';

import { create, color } from '@amcharts/amcharts4/core';
import {
  XYChart,
  XYCursor,
  LineSeries,
  DateAxis,
  ValueAxis,
  Legend,
} from '@amcharts/amcharts4/charts';

import { BotInfo, Strategy } from '../rpc/services_pb';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss']
})
export class DashboardComponent implements OnInit, AfterViewInit {

  public botInfo: BotInfo[];

  constructor(private zone: NgZone) { }

  ngAfterViewInit() {
    const txtColor = color('#ffffff');
    this.zone.runOutsideAngular(() => {
      const g = create('compare-graph', XYChart);
      g.data = [];
      for (let i = 0; i < 365; i++) {
        const time = new Date();
        time.setDate(time.getDate() - i);
        g.data.push({
          date: time,
          hodl: Math.cos(i / 12) * 100,
          bot: Math.sin(i / 12) * 100,
        });
      }

      g.cursor = new XYCursor();
      g.cursor.lineY.opacity = 0;
      g.legend = new Legend();
      g.legend.labels.template.fill = txtColor;

      // Create axes
      const dateAxis = g.xAxes.push(new DateAxis());
      dateAxis.renderer.minGridDistance = 50;
      dateAxis.renderer.grid.template.location = 0.5;
      dateAxis.startLocation = 0.5;
      dateAxis.endLocation = 0.5;
      dateAxis.renderer.grid.template.stroke = txtColor;
      dateAxis.renderer.labels.template.fill = txtColor;
      dateAxis.tooltipDateFormat = 'd MMMM';

      // Create value axis
      const valueAxis = g.yAxes.push(new ValueAxis());
      valueAxis.renderer.grid.template.stroke = txtColor;
      valueAxis.renderer.labels.template.fill = txtColor;

      const hodl = g.series.push(new LineSeries());
      hodl.dataFields.dateX = 'date';
      hodl.dataFields.valueY = 'hodl';
      hodl.strokeWidth = 1;
      hodl.tensionX = 0.5;
      hodl.fillOpacity = 0.4;
      hodl.name = 'Hodl BTC Profit';
      hodl.tooltipText = 'Hodl BTC Profit Ratio: [bold]{hodl}%[/]';

      const bot = g.series.push(new LineSeries());
      bot.dataFields.dateX = 'date';
      bot.dataFields.valueY = 'bot';
      bot.strokeWidth = 1;
      bot.tensionX = 0.5;
      bot.fillOpacity = 0.4;
      bot.name = 'Bot Trading Profit';
      bot.tooltipText = 'Bot Trading Profit Ratio: [bold]{bot}%[/]';
    });
  }

  ngOnInit(): void {
    this.botInfo = [];
    for (let i = 0; i < 5; i++) {
      const info = new BotInfo();
      info.setId(`test-bot-${i}`);
      info.setName(`Test Bot ${i}`);
      info.setStrategy(Strategy.TRAILING);
      info.setConfig(JSON.stringify({
        entryBufferPercent: 3.0 + Math.random(),
        entryTrailingPercent: 0.2,
        exitBufferPercent: 2.0 + Math.random(),
        exitTrailingPercent: 0.2,
        stopLoss: 10.0,
      }));
      this.botInfo = this.botInfo.concat(info);
    }
  }
}
