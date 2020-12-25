import { Component, OnInit } from '@angular/core';

import { MatDialog } from '@angular/material/dialog';

import { EditDialogComponent } from './edit-dialog/edit-dialog.component';
import { RespType, EditDialogData } from './edit-dialog/edit-dialog-data'

@Component({
  selector: 'app-keychain',
  templateUrl: './keychain.component.html',
  styleUrls: ['./keychain.component.scss']
})
export class KeychainComponent implements OnInit {

  constructor(private dialogOpener: MatDialog) { }

  ngOnInit(): void {}

  openEditDialog(isNew: boolean): void {
    const dialog = this.dialogOpener.open(EditDialogComponent, {
      width: '50vw',
      data: {isNew: true},
    });
    dialog.afterClosed().subscribe(this.editKeyPair);
  }

  private editKeyPair(result: EditDialogData) {
    console.log(result);
  }

}
