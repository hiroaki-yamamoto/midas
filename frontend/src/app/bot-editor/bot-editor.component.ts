import {
  Component, OnInit,
  ElementRef, ViewChild
} from '@angular/core';
import { FormGroup } from '@angular/forms';
import { editor } from 'monaco-editor';

@Component({
  selector: 'app-bot-editor',
  templateUrl: './bot-editor.component.html',
  styleUrls: ['./bot-editor.component.scss']
})
export class BotEditorComponent implements OnInit {

  @ViewChild('monacoEditor') monacoEditor: ElementRef;

  public form: FormGroup;

  constructor() {}

  ngOnInit(): void {
    this.form = new FormGroup({});
    console.log(this.monacoEditor);
  }

}
