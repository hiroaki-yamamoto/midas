import { Component } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { HttpClient } from '@angular/common/http';

import { SyncComponent } from './sync/sync.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  constructor(private dialog: MatDialog, private http: HttpClient) {}

  openSyncSetting() {
    this.dialog.open(SyncComponent, {
      minWidth: '30vw',
      maxWidth: '60vw',
    });
  }

  preventCSRF() {
    this.http.head('/token/csrf').subscribe();
  }
}
