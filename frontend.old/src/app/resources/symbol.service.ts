import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Exchanges } from '../rpc/entities_pb';

export interface IBaseCurrencies {
  symbols: String[],
}

@Injectable({
  providedIn: 'root'
})
export class SymbolService {

  constructor(private http: HttpClient) { }
  list_base_currencies(exchange: Exchanges) {
    return this.http.get(`/symbol/base/${exchange}`)
  }
  refresh<T>(exchange: Exchanges): Observable<T> {
    return this.http.post<T>(`/symbol/refresh/${exchange}`, '');
  }
}
