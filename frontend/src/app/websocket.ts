export class MidasWebSocket extends WebSocket {
  constructor(path: string, protocols?: string | string[]) {
    const loc = window.location;
    const uri = `${((loc.protocol === 'https:') ? 'wss' : 'ws')}://${loc.host}`;
    super(`${uri}${path}`, protocols);
  }
}
