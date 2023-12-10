import {
  Component, OnInit, OnDestroy, NgZone,
} from '@angular/core';
import {
  Observable,
} from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { FormGroup, FormControl } from '@angular/forms';
import { MatSnackBar } from '@angular/material/snack-bar';

import { SymbolService, IBaseCurrencies } from '../resources/symbol.service';
import { Exchanges } from '../../rpc/exchanges.zod';
import { Bot } from '../../rpc/bot.zod';

import { faSave } from '@fortawesome/free-solid-svg-icons';


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
  public baseCurrencies: IBaseCurrencies = { symbols: [] };
  public baseCurrencyEnabled = false;
  public exchanges = Object.values(Exchanges.enum);
  public saveIcon = faSave;

  private extraLib: monaco.IDisposable;
  private langModel: monaco.IDisposable;

  constructor(
    private http: HttpClient,
    private symbol: SymbolService,
    private snackbar: MatSnackBar,
    private zone: NgZone,
  ) {
  }

  getDefCode(): Observable<string> {
    return this.http.get('/assets/bot-condition.d.ts', { responseType: 'text' });
  }

  createMonacoModel(code: string): monaco.editor.ITextModel {
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
    const uri = 'ts:bot-condition.d.ts';
    this.extraLib = ts.javascriptDefaults.addExtraLib(code, uri);
    return monaco.editor.createModel(
      code, 'typescript', monaco.Uri.parse(uri),
    );
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
    document.onkeydown = this.submit(this.form);
    this.getDefCode().subscribe((code) => {
      this.zone.runOutsideAngular(() => {
        window.require(['vs/editor/editor.main'], () => {
          this.createMonacoModel(code);
        });
      });
    });
    this.http.get('/assets/bot-condition.ts', { responseType: 'text' })
      .subscribe((code: string) => {
        condition.setValue(code);
      });
  }

  ngOnDestroy(): void {
    if (this.extraLib) { this.extraLib.dispose(); }
    if (this.langModel) { this.langModel.dispose(); }
    if (document.onkeydown) { document.onkeydown = undefined; }
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

  submit(form: FormGroup): (KeyboardEvent) => void {
    return (e: KeyboardEvent) => {
      if (!(e.ctrlKey && e.key.toLowerCase() === 's')) {
        return;
      }
      e.preventDefault();
      if (form.status === 'INVALID') {
        return;
      }
      const model = Bot.parse(this.form.value);

      this.http.post('/bot/', model).subscribe(() => {
        this.snackbar.open('Bot Saved', 'Dismiss', { duration: 3000 });
      });
    }
  }

}
