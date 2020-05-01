package binance

import "context"

// SocketInterface represents websocket interface for binance.
type SocketInterface interface {
	WatchTrade(
		ctx context.Context,
		symbol string,
	) (te chan *TradeEvent, err error)
}
