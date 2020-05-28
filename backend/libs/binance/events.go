package binance

import "github.com/adshao/go-binance"

// TradeEvent is an event instance to handle a trade event.
type TradeEvent struct {
	Event *binance.WsTradeEvent
	Error error
}
