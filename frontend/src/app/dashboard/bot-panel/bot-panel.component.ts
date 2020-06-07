import {
  Component,
  OnInit,
  Input,
  ViewChild,
  ElementRef,
  NgZone,
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
import { BotInfo } from '../../rpc/services_pb';

@Component({
  selector: 'app-bot-panel',
  templateUrl: './bot-panel.component.html',
  styleUrls: ['./bot-panel.component.scss']
})
export class BotPanelComponent implements OnInit {

  @Input() bot: BotInfo;
  @ViewChild('profitGraph') profitGraph: ElementRef;
  private g: XYChart;

  constructor(private zone: NgZone) { }

  ngOnInit(): void {
  }

  open() {
    const txtColor = color('#ffffff');
    this.zone.runOutsideAngular(() => {
      this.g = create(this.profitGraph.nativeElement, XYChart);
      this.g.data = [];
      for (let i = 0; i < 365; i++) {
        const time = new Date();
        time.setDate(time.getDate() - i);
        this.g.data.push({
          date: time,
          profitPercent: Math.sin(i / 12) * 100,
        });
      }

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

      // Create profitability series.
      const bot = this.g.series.push(new LineSeries());
      bot.dataFields.dateX = 'date';
      bot.dataFields.valueY = 'profitPercent';
      bot.strokeWidth = 1;
      bot.tensionX = 0.5;
      bot.fillOpacity = 0.4;
      bot.name = 'Profit Percent';
      bot.tooltipText = 'Trading Profit Ratio: [bold]{profitPercent}%[/]';
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
