import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, map } from 'rxjs';

import { BotList } from '../../rpc/bot-list.zod';
import { BotResponse } from '../../rpc/bot-response.zod';

@Injectable({
  providedIn: 'root'
})
export class BotService {
  constructor(private http: HttpClient) { }

  get(id: string): Observable<BotResponse> {
    return this.http.get(`/bot/${id}`)
      .pipe(map((data: object) => BotResponse.parse(data)));
  }

  list(): Observable<BotResponse[]> {
    return this.http.get('/bot')
      .pipe(map((data: object) => BotList.parse(data)))
      .pipe(map((botList: BotList) => botList.bots));
  }
}
