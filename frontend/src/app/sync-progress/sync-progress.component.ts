import {
  Component,
  OnInit,
  Input,
  Output,
  EventEmitter,
  OnChanges,
} from '@angular/core';
import { HistChartProg } from '../rpc/historical_pb';

@Component({
  selector: 'app-sync-progress',
  templateUrl: './sync-progress.component.html',
  styleUrls: ['./sync-progress.component.scss']
})
export class SyncProgressComponent implements OnInit, OnChanges {
  @Input() public progress: HistChartProg.AsObject;
  @Output() public completed: EventEmitter<HistChartProg.AsObject> = new EventEmitter(true);

  constructor() { }

  ngOnInit(): void {
  }

  ngOnChanges() {
    if (this.progress.curObjectNum >= this.progress.numObjects) {
      this.completed.emit(this.progress);
    }
  }

}
