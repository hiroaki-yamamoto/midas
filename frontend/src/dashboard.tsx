import style from './dashboard.module.scss';
import OverAllGraph from './graph-overall/view';
import { IData } from './graph-overall/data.interface.ts';
import { ILegend } from './graph-overall/legend.interface.ts';
import BotSummary from './bot-summary-panel/view';

import { Bot } from './rpc/bot_pb';

function Dashboard() {

  const legend: ILegend[] = [
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

  const bots: Bot[] = (() => {
    const bots: Bot[] = [];
    for (let i = 0; i < 10; i++) {
      const bot = new Bot();
      console.log(bot.setId);
      bot.setId(i.toString());
      bot.setName(`Test Bot ${i}`);
      bots.push(bot);
    }
    return bots;
  })();

  const data: IData[] = (() => {
    let data: IData[] = [];
    const now = new Date();
    for (let i = 0; i < 365; i++) {
      const time = new Date(now.getTime());
      time.setDate(time.getDate() - i);
      data = data.concat({
        date: time,
        hodl: Math.cos(i / 12) * 100,
        bot: Math.sin(i / 12) * 100,
      });
    }
    return data;
  })();

  return (
    <>
      <section>
        <header className={style['dashboard-header']}>
          <h1>Dashboard</h1>
        </header>
        <OverAllGraph legend={legend} data={data} />
      </section>
      <section>
        <header className={style['dashboard-header']}>
          <h1>Bots</h1>
        </header>
        {
          bots.map((bot) => {
            return (<BotSummary bot={bot} />);
          })
        }
      </section>
    </>
  );
}

export default Dashboard;
