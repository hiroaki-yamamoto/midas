import { Bot } from '../rpc/bot.zod';
import { Position } from '../rpc/position.zod';
import { PositionStatus } from '../rpc/position-status.zod';
import { IData } from '../profit-graph/data.interface';

export class Ctrl {
  private bot: Bot | undefined = undefined;

  public constructor(bot: Bot) {
    this.bot = bot;
  }

  public async getData(): Promise<IData[]> {
    return new Promise((resolve) => {
      let data: IData[] = [];
      const time = new Date();
      for (let i = 0; i < 180; i++) {
        const clonedTime = new Date(time.getTime());
        clonedTime.setDate(time.getDate() - i);
        data = data.concat({
          date: clonedTime,
          realizedPercent: Math.sin(i / 12) * 100,
          unrealizedPercent: Math.sin(i / 24) * 100,
        });
      }
      resolve(data);
    });
  }

  public async getPositions(status: PositionStatus): Promise<Position[]> {
    return new Promise((resolve) => {
      let positions: Position[] = [];
      const time = new Date();
      for (let i = 0; i < 523; i++) {
        const entry_at = new Date(time.getTime());
        entry_at.setDate(time.getDate() - i);

        const exit_at = new Date(time.getTime());
        exit_at.setDate(time.getDate() - i);
        exit_at.setHours(exit_at.getHours() + 1);

        positions = positions.concat(Position.parse({
          id: i.toString(),
          bot_id: this.bot?.id,
          entry_at,
          exit_at,
          profit_amount: (Math.random() * 1000).toString(),
          profit_percent: (Math.random() * 100).toFixed(2).toString(),
          status,
          symbol: 'BTCUSDT',
          trading_amount: (Math.random() * 1000).toString(),
          valuation: (Math.random() * 1000).toString(),
        }));
      }
      resolve(positions);
    });
  }
}
