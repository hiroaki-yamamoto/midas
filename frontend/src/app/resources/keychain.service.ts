import { Observable } from 'rxjs';
import { map, tap, catchError } from 'rxjs/operators';
import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { ApiKey as APIKey } from '../../rpc/api-key.zod';
import { ApiKeyList as APIKeyList } from '../../rpc/api-key-list.zod';
import { ApiRename as APIRename } from '../../rpc/api-rename.zod';
import { InsertOneResult } from '../../rpc/insert-one-result.zod';

@Injectable({
  providedIn: 'root'
})
export class KeychainService {
  private readonly endpoint = '/keychain';
  public keys: APIKey[] = [];

  constructor(private http: HttpClient) { }

  fetch(): Observable<APIKey[]> {
    return this.http
      .get(`${this.endpoint}/`)
      .pipe(
        map((value: APIKeyList) => value.keys),
        tap((value: APIKey[]) => {
          this.keys = value;
        })
      );
  }

  add(payload: APIKey) {
    return this.http
      .post(`${this.endpoint}/`, payload)
      .pipe(tap((res: InsertOneResult) => {
        const api = { ...payload };
        api.prvKey = ('*').repeat(16);
        api.id = res.id;
        this.keys.push(api);
      }));
  }

  rename(index: number, label: string) {
    const payload: APIRename = { label }
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
