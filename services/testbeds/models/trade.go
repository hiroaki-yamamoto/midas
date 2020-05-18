package models

import "time"

// Kline define kline info
type Kline struct {
	Symbol                   string    `json:"symbol"`
	OpenAt                   time.Time `json:"openAt"`
	Open                     float64   `json:"open"`
	High                     float64   `json:"high"`
	Low                      float64   `json:"low"`
	Close                    float64   `json:"close"`
	Volume                   float64   `json:"volume"`
	CloseAt                  time.Time `json:"closeAt"`
	QuoteAssetVolume         float64   `json:"quoteAssetVolume"`
	TradeNum                 int64     `json:"tradeNum"`
	TakerBuyBaseAssetVolume  float64   `json:"takerBuyBaseAssetVolume"`
	TakerBuyQuoteAssetVolume float64   `json:"takerBuyQuoteAssetVolume"`
}
