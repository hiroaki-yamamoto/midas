package history

import (
	"strings"

	"github.com/adshao/go-binance"
	"go.mongodb.org/mongo-driver/mongo"
	"go.uber.org/zap"
	"golang.org/x/net/context"
)

// Binance represents binance historical chart data downloader.
type Binance struct {
	Logger *zap.Logger
	Col    *mongo.Collection
	Cli    *binance.Client
}

// NewBinance constructs a new instance of Binance.
func NewBinance(log *zap.Logger, col *mongo.Collection) *Binance {
	return &Binance{
		Logger: log,
		Col:    col,
		Cli:    binance.NewClient("", ""),
	}
}

// Run starts downloading Historical data.
func (me *Binance) Run(pair string) error {
	ctx := context.Background()
	info, err := me.Cli.NewExchangeInfoService().Do(ctx)
	if err != nil {
		return err
	}
	var targetSymbols []string
	if pair == "all" {
		for _, sym := range info.Symbols {
			if strings.ToUpper(sym.Status) == "TRADING" {
				targetSymbols = append(targetSymbols, sym.Symbol)
			}
		}
	} else {
		for _, sym := range info.Symbols {
			if strings.ToUpper(sym.Symbol) == strings.ToUpper(pair) &&
				strings.ToUpper(sym.Status) == "TRADING" {
				targetSymbols = append(targetSymbols, sym.Symbol)
				break
			}
		}
	}
	if len(targetSymbols) < 1 {
		return &NoSuchPair{
			Pair: pair,
		}
	}
	me.Logger.Info("Pair to fetch", zap.Any("Pairs", targetSymbols))
	return nil
}
