import {
  Component, ViewChild, ElementRef, AfterViewInit,
  NgZone,
} from '@angular/core';

@Component({
  selector: 'app-monaco-editor',
  templateUrl: './monaco-editor.component.html',
  styleUrls: ['./monaco-editor.component.scss']
})
export class MonacoEditorComponent implements AfterViewInit {
  private editor: monaco.editor.IStandaloneEditor;
  @ViewChild('monacoContainer') private cointainer: ElementRef;

  constructor(private zone: NgZone) { }

  ngAfterViewInit() {
    this.zone.runOutsideAngular(() => {
      window.require(['vs/editor/editor.main'], () => {
        this.editor = monaco.editor.create(
          this.editor.nativeElement,
          {
            theme: 'vs-dark',
            language: 'typescript',
            value: this.form.get('condition').value
          }
        );
      });
    });
  }
}
