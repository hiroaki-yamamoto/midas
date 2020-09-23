import {
  Component,
  OnInit,
  Input,
  Output,
  EventEmitter,
  OnChanges,
} from '@angular/core';
import { IHistChartProg } from './entities';

@Component({
  selector: 'app-sync-progress',
  templateUrl: './sync-progress.component.html',
  styleUrls: ['./sync-progress.component.scss']
})
export class SyncProgressComponent implements OnInit, OnChanges {
  @Input() public progress: IHistChartProg;
  @Output() public completed:EventEmitter<IHistChartProg> = new EventEmitter(true);

  constructor() { }

  ngOnInit(): void {
  }

  ngOnChanges() {
    if (this.progress.cur_object_num >= this.progress.num_objects) {
      this.completed.emit(this.progress);
    }
  }

}
