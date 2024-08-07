import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Exchanges } from '../../rpc/exchanges.zod';

export interface IBaseCurrencies {
  symbols: string[],
}

@Injectable({
  providedIn: 'root'
})
export class SymbolService {

  constructor(private http: HttpClient) { }
  list_base_currencies(exchange: Exchanges) {
    return this.http.get(`/symbol/base/${exchange.toLowerCase()}`)
  }
  refresh<T>(exchange: Exchanges): Observable<T> {
    return this.http.post<T>(`/symbol/refresh/${exchange.toLowerCase()}`, '');
  }
}
