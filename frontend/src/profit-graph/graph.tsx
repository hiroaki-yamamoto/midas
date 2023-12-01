import {
  Root, Legend, percent, Tooltip, DataProcessor
} from '@amcharts/amcharts5';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';
import am5themes_Dark from '@amcharts/amcharts5/themes/Dark';
import {
  XYChart, XYCursor, AxisRendererX, AxisRendererY,
  DateAxis, ValueAxis, LineSeries
} from '@amcharts/amcharts5/xy';

import { IData } from './data.interface';
import { IGraphLegend } from '../graph-legend.interface';
import { IGraph } from '../graph.interface';

export class Graph implements IGraph {
  public data: IData[] = [];
  public root: Root | undefined = undefined;
  private readonly series: IGraphLegend[] = [
    {
      name: 'Realized Profit Percent',
      valueField: 'realizedPercent',
      tooltip: 'Realized Profit Percent: [bold]{realizedPercent}%[/]',
    },
    {
      name: 'Unrealized Profit Percent',
      valueField: 'unrealizedPercent',
      tooltip: 'Unrealized Profit Percent: [bold]{unrealizedPercent}%[/]',
    }
  ];

  public constructor(data: IData[]) {
    this.data = data;
  }

  public draw(ref: HTMLElement | null): () => void {
    if (!ref) {
      return this.dispose;
    }
    const root = Root.new(ref);
    this.root = root;
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
    if (!cursor) {
      console.error('Cursor is not defined');
      return this.dispose;
    }
    cursor.lineY.set('visible', false);

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
    const createSeries = (
      name: string, valueField: string, tooltip: string
    ) => {
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

    this.series.forEach((series) => {
      const { name, valueField, tooltip } = series;
      const graphSeries = createSeries(name, valueField, tooltip);
      graphSeries.data.setAll(this.data);
    });
    legend.data.setAll(chart.series.values);
    chart.appear(1000, 100);
    return this.dispose;
  }

  public dispose(): void {
    if (this.root) {
      this.root.dispose();
    }
  }
}
