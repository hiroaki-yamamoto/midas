import { Component } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';

import { SyncComponent } from './sync/sync.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  constructor(private dialog: MatDialog) {}
  openSyncSetting() {
    this.dialog.open(SyncComponent, {
      minWidth: '30vw',
      maxWidth: '60vw',
    });
  }
}
