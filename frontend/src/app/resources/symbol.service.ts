import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Exchanges } from '../rpc/entities_pb';

@Injectable({
  providedIn: 'root'
})
export class SymbolService {

  constructor(private http: HttpClient) { }
  refresh<T>(exchange: Exchanges): Observable<T> {
    return this.http.post<T>(`/symbol/refresh/${exchange}`, '');
  }
}
