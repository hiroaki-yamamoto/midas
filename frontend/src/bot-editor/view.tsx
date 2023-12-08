import { useMemo, useEffect, useState } from 'react';

import { Editor } from './editor';
import style from './style.module.scss';
import { Ctrl } from './controller';

export const BotEditor = () => {
  const [defCode, setDefCode] = useState<string>('');
  const [value, setValue] = useState<string>('');

  const ctrl = useMemo(
    () => new Ctrl(setDefCode, setValue),
    [setDefCode, setValue]
  );

  useEffect(() => {
    console.log(ctrl);
  }, [ctrl]);
  return (
    <div className={style['editor-container']}>
      <Editor value={value} language="typescript" definition={defCode} />
    </div>
  );
};
