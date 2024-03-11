import {
  Component, OnInit, OnDestroy, NgZone,
} from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Observable, tap } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { FormGroup, FormControl, Validators } from '@angular/forms';
import { MatSnackBar } from '@angular/material/snack-bar';
// import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';

import { SymbolService, IBaseCurrencies } from '../resources/symbol.service';
import { Exchanges } from '../../rpc/exchanges.zod';
import { BotRequest } from '../../rpc/bot-request.zod';
import { BotResponse } from '../../rpc/bot-response.zod';
import { BotService } from '../resources/bot.service';

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
  public exchanges = Object.values(Exchanges.enum);
  public saveIcon = faSave;

  private extraLib: monaco.IDisposable;
  private langModel: monaco.IDisposable;

  constructor(
    private http: HttpClient,
    private symbol: SymbolService,
    private snackbar: MatSnackBar,
    private zone: NgZone,
    private route: ActivatedRoute,
    private botSvc: BotService,
  ) { }

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
    this.form = new FormGroup({
      name: new FormControl('', [Validators.required]),
      exchange: new FormControl('', [Validators.required]),
      baseCurrency: new FormControl({ disabled: true }),
      tradingAmount: new FormControl('', [Validators.required]),
      condition: new FormControl(),
    });
    document.onkeydown = this.submit(this.form);
    this.getDefCode().subscribe((code) => {
      this.zone.runOutsideAngular(() => {
        window.require(['vs/editor/editor.main'], () => {
          this.langModel = this.createMonacoModel(code);
        });
      });
    });
    this.http.get('/assets/bot-condition.ts', { responseType: 'text' })
      .subscribe((code: string) => {
        this.form.get('condition').setValue(code);
      });
    this.route.data
      .subscribe(
        (data: { bot: object | void; }) => {
          if (data.bot) {
            const bot = BotResponse.parse(data.bot);
            this.setBotToForm(bot);
          }
        },
      );
  }

  setBotToForm(bot: BotResponse): void {
    this.form.get('name').setValue(bot.name);
    this.form.get('tradingAmount').setValue(bot.tradingAmount);
    this.form.get('condition').setValue(bot.condition);
    this.form.get('exchange').setValue(bot.exchange);
    this.exchangeChanged().subscribe(() => {
      this.form.get('baseCurrency').setValue(bot.baseCurrency);
    });
  }

  ngOnDestroy(): void {
    if (this.extraLib) { this.extraLib.dispose(); }
    if (this.langModel) { this.langModel.dispose(); }
    if (document.onkeydown) { document.onkeydown = undefined; }
  }

  exchangeChanged(): Observable<IBaseCurrencies> {
    this.form.get('baseCurrency').disable();
    return this.symbol
      .list_base_currencies(this.form.get('exchange').value as Exchanges)
      .pipe(
        tap((baseCurrencies: IBaseCurrencies) => {
          this.baseCurrencies = baseCurrencies;
          this.form.get('baseCurrency').enable();
        })
      );
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
      const val = form.value;
      val.tradingAmount = val.tradingAmount.toString();

      const model = BotRequest.parse(val);

      this.botSvc.post(model).subscribe(() => {
        this.snackbar.open('Bot Saved', 'Dismiss', { duration: 3000 });
      });
    };
  }
}
