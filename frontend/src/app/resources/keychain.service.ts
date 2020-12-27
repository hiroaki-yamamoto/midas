import { Observable } from 'rxjs';
import { map, tap, catchError } from 'rxjs/operators';
import { Injectable } from '@angular/core';
import { HttpClient, HttpEvent } from '@angular/common/http';

import { APIKey, APIKeyList, APIRename } from '../rpc/keychain_pb';

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
        tap((value) => this.keys = value)
      );
  }

  add(payload: APIKey.AsObject) {
    return this.http
      .post(`${this.endpoint}/`, payload)
      .pipe(tap(() => this.keys.push(payload)));
  }

  rename(index: number) {
    const target = this.keys[index];
    const payload: APIRename.AsObject = {
      label: target.label,
    }
    return this.http
      .patch(`${this.endpoint}/${target.id}`, payload);
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
