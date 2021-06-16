import {
  Component, OnInit,
} from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FormGroup, FormControl } from '@angular/forms';
import { editor, Uri } from 'monaco-editor';

@Component({
  selector: 'app-bot-editor',
  templateUrl: './bot-editor.component.html',
  styleUrls: ['./bot-editor.component.scss']
})
export class BotEditorComponent implements OnInit {

  public form: FormGroup;
  public editorOption: editor.IStandaloneEditorConstructionOptions = {
    theme: 'vs-dark',
    language: 'javascript',
  };

  constructor(private http: HttpClient) {}

  monacoLoaded(): void {
    console.log('loading');
    // validation settings
    monaco.languages.typescript.javascriptDefaults.setDiagnosticsOptions({
      noSemanticValidation: true,
      noSyntaxValidation: false
    });

    // compiler options
    monaco.languages.typescript.javascriptDefaults.setCompilerOptions({
      target: monaco.languages.typescript.ScriptTarget.ES2015,
      allowNonTsExtensions: true
    });

    this.http.get('/assets/bot-condition.d.ts', { responseType: 'text' })
      .subscribe((code: string) => {
        const uri = 'ts:bot-condition.d.ts';
        monaco.languages.typescript.javascriptDefaults.addExtraLib(code, uri);
        monaco.editor.createModel(
          code, 'typescript', Uri.parse(uri)
        );
      });
  }

  ngOnInit(): void {
    const condition = new FormControl();
    this.form = new FormGroup({
      name: new FormControl(),
      reinvest: new FormControl(),
      condition,
    });
    this.http.get('/assets/bot-condition.js.txt', { responseType: 'text' })
      .subscribe((code: string) => {
        condition.setValue(code);
      });
  }

}
