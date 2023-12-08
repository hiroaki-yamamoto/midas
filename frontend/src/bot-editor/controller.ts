import { Dispatch, SetStateAction } from 'react';

import { Http } from '../http';

export class Ctrl {
  private http = new Http();
  constructor(
    setDefCode: Dispatch<SetStateAction<string>>,
    setValue: Dispatch<SetStateAction<string>>,
  ) {
    let defCode = '';
    let placeholder = '';
    this.http.get('/bot-condition.d.ts')
      .then((resp) => {
        if (!resp.body) {
          throw new Error('Failed to fetch bot condition def. file: No body.');
        }
        resp.body.getReader().read().then(({ done, value }) => {
          defCode += value;
          if (done) {
            setDefCode(defCode);
          }
        });
      });
    this.http.get('/bot-condition.ts')
      .then((resp) => {
        if (!resp.body) {
          throw new Error('Failed to fetch bot condition def. file: No body.');
        }
        resp.body.getReader().read().then(({ done, value }) => {
          placeholder += value;
          if (done) {
            setValue(placeholder);
          }
        });
      });
  }
}
