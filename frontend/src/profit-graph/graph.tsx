import { Root } from '@amcharts/amcharts5';

import { IData } from './data.interface';
import { IGraphLegend } from '../graph-legend.interface';
import { IGraph } from '../graph.interface';

export class Graph implements IGraph {
  public data: IData[] = [];
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

  public draw(ref: HTMLElement | null): () => void { }
}
