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
    private keychain: KeychainService,
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
      console.log(this.keychain);
      switch (result.type) {
        case RespType.POST:
          const key = new APIKey();
          key.setExchange(Exchanges.BINANCE);
          key.setLabel(result.data.label);
          key.setPubKey(result.data.pubKey);
          key.setPrvKey(result.data.prvKey);
          const payload = key.toObject();
          console.log(this.keychain);
          this.keychain.add(payload).subscribe();
          break;
      }
    };
  }

}