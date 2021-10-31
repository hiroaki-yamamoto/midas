declare class SymbolInfo {
  exchange: string;
  symbol: string;
  status: string;
  base: string;
  quote: string;
}

declare class PriceVolume {
  symbolInfo: SymbolInfo;
  price: number;
  volume: number;
}

declare class Tick {
  symbolInfo: SymbolInfo;
  volume: number;
  open: number;
  close: number;
  high: number;
  low: number;
}

declare function lastTick(durationMilisec: number): Tick;
declare function SMA(durationMiliSec: number): number;
declare function EMA(durationMiliSec: number): number;
declare function RSI(durationMilisec: number): number;
declare function MACD(durationMiliSec: number): number;
declare function CCI(durationMiliSec: number): number;
