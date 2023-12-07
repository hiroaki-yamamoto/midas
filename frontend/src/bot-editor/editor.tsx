import { useRef, useEffect, useState } from 'react';

import { editor as monaco } from 'monaco-editor';
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import CssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import HtmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import TsWorker from
  'monaco-editor/esm/vs/language/typescript/ts.worker?worker';

import { EditorInput } from './interface';
import style from './style.module.scss';

self.MonacoEnvironment = {
  getWorker: function (_, label) {
    switch (label) {
      case 'json':
        return new JsonWorker();
      case 'css':
      case 'scss':
      case 'less':
        return new CssWorker();
      case 'html':
      case 'handlebars':
      case 'razor':
        return new HtmlWorker;
      case 'typescript':
      case 'javascript':
        return new TsWorker();
      default:
        return new EditorWorker();
    }
  }
};

export const Editor = (input: EditorInput) => {
  const container = useRef<HTMLDivElement>(null);
  const [editor, setEditor] = useState<monaco.IStandaloneCodeEditor>();
  useEffect(() => {
    if (editor) {
      return;
    }
    setEditor(monaco.create(container.current!, {
      value: input.value,
      language: input.language,
      theme: 'vs-dark',
    }));
  }, [container, input, editor]);
  return (
    <div ref={container} className={style.editor}></div>
  );
};
