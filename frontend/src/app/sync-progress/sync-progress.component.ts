import {
  Component,
  Input,
  Output,
  EventEmitter,
} from '@angular/core';
import { Progress } from '../rpc/historical_pb';

@Component({
  selector: 'app-sync-progress',
  templateUrl: './sync-progress.component.html',
  styleUrls: ['./sync-progress.component.scss']
})
export class SyncProgressComponent {
  @Input() public progress: Progress.AsObject;
  @Output() public completed: EventEmitter<Progress.AsObject> = new EventEmitter(true);

  constructor() { }

}
