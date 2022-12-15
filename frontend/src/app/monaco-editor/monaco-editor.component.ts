import {
  Component, ViewChild, ElementRef, AfterViewInit,
  NgZone, Input, forwardRef,
} from '@angular/core';
import {
  ControlValueAccessor, NG_VALUE_ACCESSOR
} from '@angular/forms';

@Component({
  selector: 'app-monaco-editor',
  templateUrl: './monaco-editor.component.html',
  styleUrls: ['./monaco-editor.component.scss'],
  providers: [{
    provide: NG_VALUE_ACCESSOR,
    useExisting: forwardRef(() => MonacoEditorComponent),
    multi: true
  }]
})
export class MonacoEditorComponent implements AfterViewInit, ControlValueAccessor {
  private editor: monaco.editor.IStandaloneCodeEditor;
  private value: string = '';
  private registerOnChangeFn: any;
  private registerOnTouchedFn: any;
  @ViewChild('monacoContainer') private container: ElementRef;
  @Input() public language; string;

  constructor(private zone: NgZone) { }

  ngAfterViewInit() {
    this.zone.runOutsideAngular(() => {
      window.require(['vs/editor/editor.main'], () => {
        this.editor = monaco.editor.create(
          this.container.nativeElement,
          {
            theme: 'vs-dark',
            language: this.language,
            value: this.value,
          }
        );
        this.editor.onDidChangeModelContent(() => {
          this.registerOnChangeFn(this.editor.getValue());
        });
        this.editor.onDidBlurEditorWidget(this.registerOnTouchedFn);
      });
    });
  }

  writeValue(value: any): void {
    this.value = value as string || '';
  }

  registerOnChange(fn: any): void {
    this.registerOnChangeFn = fn;
  }

  registerOnTouched(fn: any): void {
    this.registerOnTouchedFn = fn;
  }
}
