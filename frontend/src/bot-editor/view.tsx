import { Editor } from './editor';
import style from './style.module.scss';

export const BotEditor = () => {
  return (
    <div className={style['editor-container']}>
      <Editor value="Hello World" language="typescript" />
    </div>
  );
};
