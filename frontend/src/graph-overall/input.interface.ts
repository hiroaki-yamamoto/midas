import { ILegend } from '../graph-legend.interface';
import { IData } from './data.interface';

export interface Input {
  legend: ILegend[];
  data: IData[];
}
