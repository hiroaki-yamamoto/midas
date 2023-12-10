import { Component, OnInit, Inject } from '@angular/core';
import { FormGroup, FormControl, Validators } from '@angular/forms';

import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';

import { faTrashAlt } from '@fortawesome/free-solid-svg-icons'
import { EditDialogData, RespType } from './edit-dialog-data';
import { KeychainService } from '../../resources/keychain.service';
import { Exchanges } from '../../rpc/entities_pb';

export interface EditDialogOption {
  index?: number,
}

@Component({
  selector: 'app-edit-dialog',
  templateUrl: './edit-dialog.component.html',
  styleUrls: ['./edit-dialog.component.scss']
})
export class EditDialogComponent implements OnInit {
  public isNew: boolean;
  public form: FormGroup;
  public trash = faTrashAlt;
  public RespType = RespType;
  public exchanges = Exchanges;

  constructor(
    @Inject(MAT_DIALOG_DATA) public option: EditDialogOption,
    private dialog: MatDialogRef<EditDialogComponent>,
    private keychain: KeychainService,
  ) {
    this.isNew = Boolean(
      option.index === undefined ||
      option.index === null ||
      option.index < 0
    );
  }

  ngOnInit(): void {
    const onlyNewValidation = (this.isNew) && Validators.required || undefined;
    this.form = new FormGroup({
      id: new FormControl('',),
      exchange: new FormControl('', Validators.required),
      label: new FormControl('', Validators.required),
      pubKey: new FormControl('', onlyNewValidation),
      prvKey: new FormControl('', onlyNewValidation)
    });
    if (this.isNew) {
      this.form.get('exchange').setValue(Exchanges.BINANCE);
    } else {
      this.form.get('pubKey').disable();
      this.form.get('prvKey').disable();
      this.form.get('exchange').disable();
      this.form.setValue(
        { ...this.keychain.keys[this.option.index] },
      );
    }
  }

  close(type: RespType) {
    const respData: EditDialogData = { type };
    if (type == RespType.POST) {
      respData.data = this.form.value;
    }
    if (!this.isNew) {
      respData.index = this.option.index;
    }
    this.dialog.close(respData);
  }

}
