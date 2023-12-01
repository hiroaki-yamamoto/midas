import style from './dashboard.module.scss';
import OverAllGraph from '../graph-overall/view.tsx';
import { IData } from '../graph-overall/data.interface.ts';
import { ILegend } from '../graph-overall/legend.interface.ts';
import { Bot } from '../rpc/bot.zod.ts';
import { Exchanges } from '../rpc/exchanges.zod.ts';
import { dateToTimestamp } from '../timestamp-utils.ts';
import BotPanel from '../bot-panel/bot-panel.tsx';

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

  const bots: Bot[] = (() => {
    let bots: Bot[] = [];
    const baseTime = new Date(
      (new Date()).getTime() - 36000000,
    ); // 10 hours ago
    for (let i = 0; i < 10; i++) {
      const time = dateToTimestamp(new Date(baseTime.getTime() + 3600000 * i));
      const info = Bot.parse({
        base_currency: 'USDT',
        condition: 'ACTIVE',
        created_at: time,
        exchange: Exchanges.enum.Binance,
        id: `test-bot-${i}`,
        name: `Test Bot ${i}`,
        trading_amount: (Math.random() * 10000).toFixed(2).toString(),
      });
      bots = bots.concat(info);
    }
    return bots;
  })();

  const botsAccordions = bots.map((bot) => {
    return (
      <BotPanel bot={bot} />
    );
  });

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
        {botsAccordions}
      </section>
    </>
  );
}

export default Dashboard;
