import { useMemo, useEffect } from 'react';

import { Editor } from './editor';
import style from './style.module.scss';
import { Ctrl } from './controller';

export const BotEditor = () => {
  const ctrl = useMemo(() => new Ctrl(), []);
  useEffect(() => {
    console.log(ctrl);
  }, [ctrl]);
  return (
    <div className={style['editor-container']}>
      <Editor value="Hello World" language="typescript" />
    </div>
  );
};
