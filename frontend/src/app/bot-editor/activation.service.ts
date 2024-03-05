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
      return of(true);
    }
    return this.bot_svc.get(botId).pipe(map(
      (bot: BotResponse) => {
        if (!bot) {
          return false;
        }
        route.data.bot = bot;
        return true;
      }
    ));
  }
}
