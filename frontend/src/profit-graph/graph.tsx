import { Root, Legend, percent } from '@amcharts/amcharts5';
import am5themes_Animated from '@amcharts/amcharts5/themes/Animated';
import am5themes_Dark from '@amcharts/amcharts5/themes/Dark';
import {
  XYChart, XYCursor,
  DateAxis, ValueAxis
} from '@amcharts/amcharts5/xy';

import { IData } from './data.interface';
import { IGraphLegend } from '../graph-legend.interface';
import { IGraph } from '../graph.interface';

export class Graph implements IGraph {
  public data: IData[] = [];
  public root: Root | undefined = undefined;
  private readonly legend: IGraphLegend[] = [
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
    chart.get('cursor').lineY.set('visible', false);

    const legend = chart.children.push(Legend.new(root, {
      x: percent(50),
      centerX: percent(50),
    }));
  }

  public dispose(): void {
    if (this.root) {
      this.root.dispose();
    }
  }
}
