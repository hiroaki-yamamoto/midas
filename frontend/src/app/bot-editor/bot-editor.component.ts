import {
  Component, OnInit,
} from '@angular/core';
import { FormGroup } from '@angular/forms';

@Component({
  selector: 'app-bot-editor',
  templateUrl: './bot-editor.component.html',
  styleUrls: ['./bot-editor.component.scss']
})
export class BotEditorComponent implements OnInit {

  public form: FormGroup;
  public editorOption = {
    theme: 'vs-dark',
    language: 'javascript'
  };

  constructor() {}

  ngOnInit(): void {
    this.form = new FormGroup({});
  }

}
