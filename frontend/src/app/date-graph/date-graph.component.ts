import {
  Component,
  AfterViewInit,
  ViewChild,
  ElementRef,
  OnDestroy,
  Input
} from '@angular/core';

import * as am from '@amcharts/amcharts5';
import * as amxy from '@amcharts/amcharts5/xy';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';
import am5themes_Dark from '@amcharts/amcharts5/themes/Dark';

import { BrowserOnlyService } from '../browser-only.service';

export interface ISeries {
  name: string;
  valueField: string;
  tooltip: string;
}

@Component({
  selector: 'app-date-graph',
  templateUrl: './date-graph.component.html',
  styleUrls: ['./date-graph.component.scss']
})
export class DateGraphComponent implements AfterViewInit, OnDestroy {

  @ViewChild('dateGraph') private graph: ElementRef;
  @Input() public graphData: object[];
  @Input() public series: ISeries[];

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
          tooltip: am.Tooltip.new(root, { labelText: tooltip }),
        }));
        ret.data.processor = am.DataProcessor.new(root, {
          dateFormat: 'yyyy-MM-dd',
          dateFields: ['date'],
        });
        return ret
      };

      this.series.forEach((series) => {
        const { name, valueField, tooltip } = series;
        const graphSeries = createSeries(name, valueField, tooltip);
        graphSeries.data.setAll(this.graphData);
      });
      legend.data.setAll(chart.series.values);
      chart.appear(1000, 100);
    });
  }

  ngOnDestroy() {
    this.browserOnly.browserOnly(() => {
      if (this.chartRoot) {
        this.chartRoot.dispose()
      }
    });
  }

}
