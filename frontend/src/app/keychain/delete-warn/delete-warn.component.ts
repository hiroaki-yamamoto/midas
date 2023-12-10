import { Component, Inject } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { IDialogData } from './idialog-data';

@Component({
  selector: 'app-delete-warn',
  templateUrl: './delete-warn.component.html',
  styleUrls: ['./delete-warn.component.scss']
})
export class DeleteWarnComponent {

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: IDialogData,
    private dialog: MatDialogRef<DeleteWarnComponent>,
  ) { }
}
