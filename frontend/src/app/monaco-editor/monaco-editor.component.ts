import {
  Component, ViewChild, ElementRef, AfterViewInit,
  NgZone, Input, forwardRef, OnDestroy
} from '@angular/core';
import {
  ControlValueAccessor, NG_VALUE_ACCESSOR
} from '@angular/forms';
// import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';

@Component({
  standalone: false,
  selector: 'app-monaco-editor',
  templateUrl: './monaco-editor.component.html',
  styleUrls: ['./monaco-editor.component.scss'],
  providers: [{
    provide: NG_VALUE_ACCESSOR,
    useExisting: forwardRef(() => MonacoEditorComponent),
    multi: true
  }]
})
export class MonacoEditorComponent implements
  AfterViewInit, ControlValueAccessor, OnDestroy {
  private editor: monaco.editor.IStandaloneCodeEditor | undefined;
  private value: string = '';
  private registerOnChangeFn: (string) => void;
  private registerOnTouchedFn: () => void;
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

  ngOnDestroy(): void {
    this.editor?.dispose();
  }

  writeValue(value: string): void {
    this.value = value || '';
    this.zone.runOutsideAngular(() => {
      window.require(['vs/editor/editor.main'], () => {
        this.editor?.setValue(this.value);
      });
    });
  }

  registerOnChange(fn: (string) => void): void {
    this.registerOnChangeFn = fn;
  }

  registerOnTouched(fn: () => void): void {
    this.registerOnTouchedFn = fn;
  }

  setDisabledState(isDisabled: boolean): void {
    this.editor?.updateOptions({ readOnly: isDisabled });
  }
}
