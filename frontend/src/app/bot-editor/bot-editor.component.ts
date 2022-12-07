import {
  Component, OnInit, OnDestroy
} from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FormGroup, FormControl } from '@angular/forms';
import { MatSnackBar } from '@angular/material/snack-bar';
import * as monaco from 'monaco-editor';

import { SymbolService, IBaseCurrencies } from '../resources/symbol.service';
import { Exchanges } from '../rpc/entities_pb';
import { Bot } from '../rpc/bot_pb';
import { require } from '../js-declare';

import { faSave } from '@fortawesome/free-solid-svg-icons'

@Component({
  selector: 'app-bot-editor',
  templateUrl: './bot-editor.component.html',
  styleUrls: ['./bot-editor.component.scss']
})
export class BotEditorComponent implements OnInit, OnDestroy {

  public form: FormGroup;
  public editorOption: monaco.editor.IStandaloneEditorConstructionOptions = {
    theme: 'vs-dark',
    language: 'typescript',
    tabSize: 2,
  };
  public baseCurrencies: IBaseCurrencies = { currencies: [] };
  public baseCurrencyEnabled = false;
  public exchanges = Object.values(Exchanges);
  public saveIcon = faSave;

  private extraLib: monaco.IDisposable;
  private langModel: monaco.IDisposable

  constructor(
    private http: HttpClient,
    private symbol: SymbolService,
    private snackbar: MatSnackBar
  ) {
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
          code, 'typescript', monaco.Uri.parse(uri),
        );
      });
  }

  ngOnInit(): void {
    const condition = new FormControl();
    this.form = new FormGroup({
      name: new FormControl(),
      exchange: new FormControl(),
      baseCurrency: new FormControl(),
      tradingAmount: new FormControl(),
      condition,
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

  exchangeChanged(): void {
    this.baseCurrencyEnabled = false;
    this.symbol
      .list_base_currencies(this.form.get('exchange').value)
      .subscribe((baseCurrencies: IBaseCurrencies) => {
        this.baseCurrencies = baseCurrencies;
      });
    this.baseCurrencyEnabled = true;
  }

  submit(): void {
    const model = new Bot();
    model.setName(this.form.get('name').value);
    model.setExchange(this.form.get('exchange').value);
    model.setBaseCurrency(this.form.get('baseCurrency').value);
    model.setTradingAmount(this.form.get('tradingAmount').value);
    model.setCondition(this.form.get('condition').value);
    this.http.post('/bot/', model.toObject()).subscribe(() => {
      this.snackbar.open('Bot Saved', 'Dismiss', { duration: 3000 });
    });
  }

}
