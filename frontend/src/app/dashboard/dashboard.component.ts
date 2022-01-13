import {
  Component,
  OnInit,
  ViewChild,
  ElementRef,
} from '@angular/core';

import { Bot } from '../rpc/bot_pb';
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
  public series: ISeries[] = [
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
    this.botsInfo = [];
    for (let i = 0; i < 5; i++) {
      const info = new Bot();
      info.setId(`test-bot-${i}`);
      info.setName(`Test Bot ${i}`);
      this.botsInfo = this.botsInfo.concat(info);
    }
  }
}
