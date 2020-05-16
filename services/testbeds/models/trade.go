package models

import "time"

// Kline define kline info
type Kline struct {
	Symbol                   string    `json:"symbol"`
	OpenTime                 time.Time `json:"openTime"`
	Open                     float64   `json:"open"`
	High                     float64   `json:"high"`
	Low                      float64   `json:"low"`
	Close                    float64   `json:"close"`
	Volume                   float64   `json:"volume"`
	CloseTime                time.Time `json:"closeTime"`
	QuoteAssetVolume         float64   `json:"quoteAssetVolume"`
	TradeNum                 int64     `json:"tradeNum"`
	TakerBuyBaseAssetVolume  float64   `json:"takerBuyBaseAssetVolume"`
	TakerBuyQuoteAssetVolume float64   `json:"takerBuyQuoteAssetVolume"`
}
