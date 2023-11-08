import { Bot, Position } from '../rpc/bot_pb';

export interface Input {
  /** Bot summary */
  bot: Bot,
  /** Current Positions */
  curPos: Position[],
  /** Archived Positions */
  arcPos: Position[],
}
