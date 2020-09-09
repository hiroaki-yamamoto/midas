import { Component, OnInit } from '@angular/core';
import { faTimes, faSyncAlt, faHistory } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-sync',
  templateUrl: './sync.component.html',
  styleUrls: ['./sync.component.scss']
})
export class SyncComponent implements OnInit {
  closeIcon = faTimes;
  syncIcon = faSyncAlt;
  histIcon = faHistory;

  constructor() { }

  ngOnInit(): void {
  }

}
