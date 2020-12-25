import { Component, OnInit, Inject } from '@angular/core';
import { FormGroup, FormControl, Validators } from '@angular/forms';

import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';

import { faTrashAlt } from '@fortawesome/free-solid-svg-icons'
import { EditDialogData, RespType } from './edit-dialog-data';

export interface EditDialogOption {
  isNew?: boolean
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

  constructor(
    @Inject(MAT_DIALOG_DATA) public option: EditDialogOption,
    private dialog: MatDialogRef<EditDialogComponent>,
  ) {
    this.isNew = Boolean(option.isNew);
  }

  ngOnInit(): void {
    const onlyNewValidation = (this.isNew) && Validators.required || undefined;
    this.form = new FormGroup({
      label: new FormControl('', Validators.required),
      pubKey: new FormControl('', onlyNewValidation),
      prvKey: new FormControl('', onlyNewValidation)
    });
    if (!this.isNew) {
      this.form.get('pubKey').disable();
      this.form.get('prvKey').disable();
    }
  }

  close(type: RespType) {
    let respData: EditDialogData = { type };
    if (type == RespType.POST) {
      respData.data = this.form.value;
    }
    this.dialog.close(respData);
  }

}
