package strategies

// Portfolio indicates a portfolio summary.
// Symbol as a key, and the amount as a value.
type Portfolio map[string]float64

// IBot represents a trading bot.
type IBot interface {
	GetName() string
	GetTradingAmount() string
	GetBaseCurrency() string
	GetPortfolio() Portfolio
	GetTotalProfit() float64
	Start() error
	Stop() error
}
