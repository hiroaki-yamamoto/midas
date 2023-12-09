import { Observable } from 'rxjs';
import { map, tap, catchError } from 'rxjs/operators';
import { Injectable } from '@angular/core';
import { HttpClient, HttpEvent } from '@angular/common/http';

import { APIKey, APIKeyList, APIRename } from '../rpc/keychain_pb';
import { InsertOneResult } from '../rpc/entities_pb';

@Injectable({
  providedIn: 'root'
})
export class KeychainService {
  private readonly endpoint = '/keychain';
  public keys: APIKey.AsObject[] = [];

  constructor(private http: HttpClient) { }

  fetch(): Observable<APIKey.AsObject[]> {
    return this.http
      .get(`${this.endpoint}/`)
      .pipe(
        map((value: APIKeyList.AsObject) => value.keysList),
        tap((value: APIKey.AsObject[]) => {
          this.keys = value;
        })
      );
  }

  add(payload: APIKey.AsObject) {
    return this.http
      .post(`${this.endpoint}/`, payload)
      .pipe(tap((res: InsertOneResult.AsObject) => {
        let api = {...payload};
        api.prvKey = ('*').repeat(16);
        api.id = res.id;
        this.keys.push(api);
      }));
  }

  rename(index: number, label: string) {
    const payload: APIRename.AsObject = {label}
    return this.http
      .patch(`${this.endpoint}/${this.keys[index].id}`, payload)
      .pipe(tap(() => {
        this.keys[index].label = label;
      }));
  }

  delete(index: number) {
    const target = this.keys.splice(index, 1)[0];
    return this.http
      .delete(`${this.endpoint}/${target.id}`)
      .pipe(catchError((e) => {
        this.keys.splice(index, 0, target);
        throw e;
      }));
  }
}
