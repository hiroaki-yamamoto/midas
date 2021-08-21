import {
  Component, OnInit, OnDestroy
} from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FormGroup, FormControl } from '@angular/forms';
import { editor, Uri, IDisposable } from 'monaco-editor';

import { SymbolService, IBaseCurrencies } from '../resources/symbol.service';
import { Exchanges } from '../rpc/entities_pb';

@Component({
  selector: 'app-bot-editor',
  templateUrl: './bot-editor.component.html',
  styleUrls: ['./bot-editor.component.scss']
})
export class BotEditorComponent implements OnInit, OnDestroy {

  public form: FormGroup;
  public editorOption: editor.IStandaloneEditorConstructionOptions = {
    theme: 'vs-dark',
    language: 'typescript',
    tabSize: 2,
  };
  public baseCurrencies: IBaseCurrencies;
  public exchanges = Object.values(Exchanges);

  private extraLib: IDisposable;
  private langModel: IDisposable

  constructor(private http: HttpClient, private symbol: SymbolService) {
    symbol.list_base_currencies(Exchanges.BINANCE).subscribe((baseCurrencies: IBaseCurrencies) => {
      this.baseCurrencies = baseCurrencies;
    });
  }

  monacoLoaded(): void {
    const ts = monaco.languages.typescript;
    // validation settings
    ts.javascriptDefaults.setDiagnosticsOptions({
      noSemanticValidation: true,
      noSyntaxValidation: false
    });

    // compiler options
    ts.javascriptDefaults.setCompilerOptions({
      target: ts.ScriptTarget.ES2015,
      allowNonTsExtensions: true
    });

    this.http.get('/assets/bot-condition.d.ts', { responseType: 'text' })
      .subscribe((code: string) => {
        const uri = 'ts:bot-condition.d.ts';
        this.extraLib = ts.javascriptDefaults.addExtraLib(code, uri);
        this.langModel = monaco.editor.createModel(
          code, 'typescript', Uri.parse(uri),
        );
      });
  }

  ngOnInit(): void {
    const condition = new FormControl();
    this.form = new FormGroup({
      name: new FormControl(),
      condition,
      baseCurrency: new FormControl(),
      tradingAmount: new FormControl(),
    });
    this.http.get('/assets/bot-condition.ts', { responseType: 'text' })
      .subscribe((code: string) => {
        condition.setValue(code);
      });
  }

  ngOnDestroy(): void {
    if (this.extraLib) { this.extraLib.dispose(); }
    if (this.langModel) { this.langModel.dispose(); }
  }

}
