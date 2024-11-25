import {
  Component,
  OnInit,
} from '@angular/core';

import { BotResponse } from '../../rpc/bot-response.zod';
import { BotService } from '../resources/bot.service';
import { ISeries } from '../date-graph/date-graph.component';

interface IGraphData {
  date: Date,
  hodl: number,
  bot: number,
}

class BotInfoHandler {
  public bots: BotResponse[];

  constructor() {
    this.bots = [];
  }

  public next(value: BotResponse[]) {
    this.bots = value || [];
  }

  public error(err: Error) {
    console.error(err);
  }
}

@Component({
  standalone: false,
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss']
})
export class DashboardComponent implements OnInit {

  public botsInfoHandler = new BotInfoHandler();
  public data: IGraphData[];
  public readonly series: ISeries[] = [
    {
      name: 'Hodl BTC Profit',
      valueField: 'hodl',
      tooltip: 'Hodl BTC Profit Ratio: [bold]{valueY}%[/]',
    },
    {
      name: 'Bot Trading Profit',
      valueField: 'bot',
      tooltip: 'Bot Trading Profit Ratio: [bold]{valueY}%[/]',
    },
  ];

  constructor(private botSvc: BotService) {
    this.data = [];
    const now = new Date();
    for (let i = 0; i < 365; i++) {
      const time = new Date(now.getTime());
      time.setDate(time.getDate() - i);
      this.data.push({
        date: time,
        hodl: Math.cos(i / 12) * 100,
        bot: Math.sin(i / 12) * 100,
      });
    }
  }

  ngOnInit(): void {
    this.botSvc.list().subscribe(this.botsInfoHandler);
  }
}
