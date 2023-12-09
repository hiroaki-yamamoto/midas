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
    Promise.all([
      this.http.get('/bot-condition.d.ts'),
      this.http.get('/bot-condition.ts'),
    ]).then(([defResp, valueResp]) => {
      return Promise.all([defResp.text(), valueResp.text()]);
    }).then(([defCode, value]) => {
      this.defCode += defCode;
      setDefCode(this.defCode);
      this.value += value;
      setCond(this.value);
    }).catch((err) => {
      console.error(err);
      return;
    });
  }
}
