import { useMemo, useState } from 'react';

import { Editor } from '../editor/view';
import style from './style.module.scss';
import { Ctrl } from './controller';

export const BotEditor = () => {
  const [defCode, setDefCode] = useState('');
  const [cond, setCond] = useState('');
  useMemo(
    () => new Ctrl(setDefCode, setCond),
    [setDefCode, setCond]
  );

  return (
    <div className={style['editor-container']}>
      <Editor
        value={cond}
        language="typescript"
        definition={defCode}
      />
    </div>
  );
};
