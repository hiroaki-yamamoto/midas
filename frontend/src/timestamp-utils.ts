import { Timestamp } from './rpc/timestamp.zod.ts';

export function timestampToDate(timestamp: Timestamp): Date {
  return new Date((timestamp.secs * 1000) + (timestamp.nanos / 1_000_000));
}

export function dateToTimestamp(date: Date): Timestamp {
  return {
    secs: Math.floor(date.getTime() / 1000),
    nanos: (date.getTime() % 1000) * 1_000_000,
  };
}
