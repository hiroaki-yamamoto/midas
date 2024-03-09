import { Injectable } from '@angular/core';
import { Observable, of, map } from 'rxjs';
import { ActivatedRouteSnapshot } from '@angular/router';

import { BotService } from '../resources/bot.service';
import { BotResponse } from '../../rpc/bot-response.zod';

@Injectable({
  providedIn: 'root'
})
export class ActivationService {
  constructor(private bot_svc: BotService) { }

  public canActivate(
    route: ActivatedRouteSnapshot,
  ): Observable<boolean> {
    const botId = route.params.botId;
    if (!botId) {
      delete route.data.bot;
      return of(true);
    }
    return this.bot_svc.get(botId, 'Detail')
      .pipe(map((bot: BotResponse) => {
        if (bot) {
          const data = {
            ...route.data,
            bot: bot,
          };
          route.data = Object.preventExtensions(data);
          return true;
        }
        return false;
      }));
  }
}
