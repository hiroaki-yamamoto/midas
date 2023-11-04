import {
  Root as Chart, Legend,
  percent, Tooltip, DataProcessor
} from '@amcharts/amcharts5';
import {
  XYChart, XYCursor, DateAxis, AxisRendererX,
  AxisRendererY, ValueAxis, LineSeries
} from '@amcharts/amcharts5/xy';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';
import am5themes_Dark from '@amcharts/amcharts5/themes/Dark';

import { ILegend } from './legend.interface';
import { IData } from './data.interface';

export class Graph {
  public legend: ILegend[];
  public data: IData[];

  constructor(legend: ILegend[], data: IData[]) {
    this.legend = legend;
    this.data = data;
  }

  /**
   * Draw the graph, and return a function that can be used to
   * destroy the graph.
   */
  public draw(ref: HTMLElement | null): () => void {
    if (!ref) {
      return () => { };
    }

    const root = Chart.new(ref);
    root.setThemes([
      am5themes_Animated.new(root),
      am5themes_Dark.new(root),
    ]);
    const chart = root.container.children.push(
      XYChart.new(root, {
        layout: root.verticalLayout,
        panY: true,
        panX: true,
        wheelX: 'panX',
        wheelY: 'zoomX',
        cursor: XYCursor.new(root, {}),
      })
    );
    const cursor = chart.get('cursor');
    if (cursor) {
      cursor.lineY.set('visible', false);
    }
    const legend = chart.children.push(Legend.new(root, {
      x: percent(50),
      centerX: percent(50),
    }));

    // Create axes
    const dateAxis = chart.xAxes.push(DateAxis.new(root, {
      renderer: AxisRendererX.new(root, {}),
      baseInterval: { timeUnit: 'day', count: 1 },
      tooltip: Tooltip.new(root, {}),
      tooltipDateFormat: 'yyyy-MM-dd'
    }));

    // Create value axis
    const valueAxis = chart.yAxes.push(ValueAxis.new(root, {
      renderer: AxisRendererY.new(root, {}),
    }));

    // Create line series
    const createSeries =
      (name: string, valueField: string, tooltip: string) => {
        const ret = chart.series.push(LineSeries.new(root, {
          name,
          valueXField: 'date',
          valueYField: valueField,
          xAxis: dateAxis,
          yAxis: valueAxis,
          tooltip: Tooltip.new(root, { labelText: tooltip }),
        }));
        ret.data.processor = DataProcessor.new(root, {
          dateFormat: 'yyyy-MM-dd',
          dateFields: ['date'],
        });
        return ret;
      };

    this.legend.forEach((series) => {
      const { name, valueField, tooltip } = series;
      const graphSeries = createSeries(name, valueField, tooltip);
      graphSeries.data.setAll(this.data);
    });
    legend.data.setAll(chart.series.values);
    chart.appear(1000, 100);
    return () => root.dispose();
  }
}
