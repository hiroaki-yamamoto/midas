import { Observable } from 'rxjs';
import { map, tap } from 'rxjs/operators';
import { Injectable } from '@angular/core';
import { HttpClient, HttpEvent } from '@angular/common/http';

import { APIKey, APIKeyList, APIRename } from '../rpc/keychain_pb';
import { Exchanges } from '../rpc/entities_pb';

@Injectable({
  providedIn: 'root'
})
export class KeychainService {
  private readonly endpoint = '/keychain';
  public keys: APIKey.AsObject[] = [];

  constructor(private http: HttpClient) { }

  fetch(exchange: Exchanges): Observable<APIKey.AsObject[]> {
    return this.http
      .get(`${this.endpoint}/${exchange.toString()}`)
      .pipe(
        map((value: APIKeyList.AsObject) => value.keysList),
        tap((value) => this.keys = value)
      );
  }

  add(
    exchange: Exchanges,
    payload: APIKey.AsObject
  ) {
    return this.http
      .post(`${this.endpoint}/${exchange.toString()}`, payload)
      .pipe(tap(() => this.keys.push(payload)));
  }

  rename(exchange: Exchanges, index: number) {
    const payload: APIRename.AsObject = {
      label: this.keys[index].label,
    }
    return this.http
      .patch(`${this.endpoint}/${exchange.toString()}`, payload);
  }

  delete(exchange: Exchanges, index: number) {
    return this.http
      .delete(`${this.endpoint}/${exchange.toString()}`)
      .pipe(tap(() => this.keys.splice(index, 1)));
  }
}
