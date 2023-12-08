import { Dispatch, SetStateAction } from 'react';
import { Http } from '../http';

export class Ctrl {
  private http = new Http();
  private defCode = '';
  private value = '';
  constructor(
    setDefCode: Dispatch<SetStateAction<string>>,
    setCond: Dispatch<SetStateAction<string>>,
  ) {
    this.http.get('/bot-condition.d.ts')
      .then((resp) => {
        if (!resp.body) {
          throw new Error('Failed to fetch bot condition def. file: No body.');
        }
        resp.text().then((txt) => {
          if (!txt) {
            return;
          }
          this.defCode += txt;
          setDefCode(this.defCode);
        });
      });
    this.http.get('/bot-condition.ts')
      .then((resp) => {
        if (!resp.body) {
          throw new Error('Failed to fetch bot condition def. file: No body.');
        }
        resp.text().then((txt) => {
          if (!txt) {
            return;
          }
          this.value += txt;
          setCond(this.value);
        });
      });
  }
}
