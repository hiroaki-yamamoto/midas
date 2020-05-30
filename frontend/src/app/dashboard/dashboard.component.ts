import {
  Component,
  OnInit,
  NgZone,
  AfterViewInit,
} from '@angular/core';

import { create } from '@amcharts/amcharts4/core';
import { XYChart, LineSeries, DateAxis, ValueAxis } from '@amcharts/amcharts4/charts';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss']
})
export class DashboardComponent implements OnInit, AfterViewInit {

  constructor(private zone: NgZone) { }

  ngAfterViewInit() {
    this.zone.runOutsideAngular(() => {
      const g = create('compare-graph', XYChart);
      g.data = [];
      for (let i = 0; i < 365; i++) {
        const time = new Date();
        time.setDate(time.getDate() - i);
        g.data.push({
          date: time,
          hodl: Math.cos(i/12),
          bot: Math.sin(i/12),
        });
      }

      // Create axes
      const dateAxis = g.xAxes.push(new DateAxis());
      dateAxis.renderer.minGridDistance = 50;
      dateAxis.renderer.grid.template.location = 0.5;
      dateAxis.startLocation = 0.5;
      dateAxis.endLocation = 0.5;

      // Create value axis
      const valueAxis = g.yAxes.push(new ValueAxis());

      const hodl = g.series.push(new LineSeries());
      hodl.dataFields.dateX = 'date';
      hodl.dataFields.valueY = 'hodl';
      hodl.strokeWidth = 1;
      hodl.tensionX = 0.5;

      const bot = g.series.push(new LineSeries());
      bot.dataFields.dateX = 'date';
      bot.dataFields.valueY = 'bot';
      bot.strokeWidth = 1;
      bot.tensionX = 0.5;
    });
  }

  ngOnInit(): void {
  }

}
