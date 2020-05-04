package models

import "time"

// TradeType represents the type of the trade.
type TradeType string

const (
	// BUY trade was made by the taker
	BUY TradeType = "Buy"
	// SELL trade was made by the taker
	SELL TradeType = "Sell"
)

// Trade represents trade data on the DB.
type Trade struct {
	Symbol    string
	Timestamp time.Time
	Type      TradeType
	Price     float64
	Qty       float64
}
