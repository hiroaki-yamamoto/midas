import {
  Component,
  OnInit,
} from '@angular/core';

import { Bot } from '../../rpc/bot.zod';
import { Exchanges } from '../../rpc/exchanges.zod';
import { ISeries } from '../date-graph/date-graph.component'

interface IGraphData {
  date: Date,
  hodl: number,
  bot: number,
}

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.scss']
})
export class DashboardComponent implements OnInit {

  public botsInfo: Bot[];
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

  constructor() {
    this.data = [];
    this.botsInfo = [];
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
    for (let i = 0; i < 5; i++) {
      const info = Bot.parse({
        id: `test-bot-${i}`,
        name: `Test Bot ${i}`,
        exchange: Exchanges.enum.Binance,
        baseCurrency: 'USDT',
        tradingAmount: '12000',
      });
      this.botsInfo = this.botsInfo.concat(info);
    }
  }
}
