import { Component, OnInit } from '@angular/core';

import { MatDialog } from '@angular/material/dialog';

import { EditDialogComponent } from './edit-dialog/edit-dialog.component';
import { RespType, EditDialogData } from './edit-dialog/edit-dialog-data'

import { KeychainService } from '../resources/keychain.service';
import { Exchanges } from '../rpc/entities_pb';
import { APIKey } from '../rpc/keychain_pb';

@Component({
  selector: 'app-keychain',
  templateUrl: './keychain.component.html',
  styleUrls: ['./keychain.component.scss']
})
export class KeychainComponent implements OnInit {

  constructor(
    private dialogOpener: MatDialog,
    public keychain: KeychainService,
  ) { }

  ngOnInit(): void {}

  openEditDialog(index?: number): void {
    const dialog = this.dialogOpener.open(EditDialogComponent, {
      width: '50vw',
      data: {index},
    });
    dialog.afterClosed().subscribe(this.editKeyPair());
  }

  private editKeyPair() {
    return (result: EditDialogData) => {
      if (result === undefined || result === null) {
        return;
      }
      switch (result.type) {
        case RespType.POST:
          if (result.index >= 0) {
            this.keychain.rename(result.index, result.data.label).subscribe();
          } else {
            const key = new APIKey();
            key.setExchange(result.data.exchange);
            key.setLabel(result.data.label);
            key.setPubKey(result.data.pubKey);
            key.setPrvKey(result.data.prvKey);
            const payload = key.toObject();
            this.keychain.add(payload).subscribe();
          }
          break;
        case RespType.CANCEL:
          break;
      }
    };
  }

}
