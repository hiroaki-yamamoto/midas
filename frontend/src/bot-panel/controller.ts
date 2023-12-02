import { Bot } from '../rpc/bot.zod';
// import { Position } from '../rpc/position.zod';
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
}
