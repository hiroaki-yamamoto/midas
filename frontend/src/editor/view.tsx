import { useRef, useEffect, useState, useMemo } from 'react';

import { editor as monaco } from 'monaco-editor';
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import CssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import HtmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import TsWorker from
  'monaco-editor/esm/vs/language/typescript/ts.worker?worker';

import { EditorInput } from './interface';
import { Ctrl } from './controller';
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
  const controller = useMemo(() => new Ctrl(), []);
  const container = useRef<HTMLDivElement>(null);
  const [editor, setEditor] = useState<monaco.IStandaloneCodeEditor>();
  useEffect(() => {
    if (!input.definition) {
      return;
    }
    switch (input.language) {
      case 'typescript':
        controller.setTsDefModel(input.definition);
        break;
    }
  }, [input, controller]);
  useEffect(() => {
    if (!input.value) {
      return;
    }
    if (editor) {
      editor.setValue(input.value);
    } else {
      setEditor(monaco.create(container.current!, {
        language: input.language,
        value: input.value,
        theme: 'vs-dark',
      }));
    }
  }, [container, input, editor, setEditor]);
  return (
    <div ref={container} className={style.editor}></div>
  );
};
