package history

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"time"

	"github.com/adshao/go-binance"
	"github.com/adshao/go-binance/common"
	"github.com/bitly/go-simplejson"
	"github.com/hiroaki-yamamoto/midas/services/testbeds/models"
	"go.mongodb.org/mongo-driver/bson"
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

func (me *Binance) fetch(
	pair string,
	startTime, endTime time.Time,
) ([]*models.Kline, error) {
	const endpoint = "https://api.binance.com/api/v3/klines"
	query := make(url.Values)
	query.Set("symbol", pair)
	query.Set("interval", "1m")
	query.Set("startTime", strconv.FormatInt(startTime.Unix(), 10))
	query.Set("endTime", strconv.FormatInt(endTime.Unix(), 10))
	query.Set("limit", "1000")
	for {
		resp, err := http.Get(fmt.Sprintf("%s?%s", endpoint, query.Encode()))
		if err != nil {
			return nil, err
		}
		switch resp.StatusCode {
		case http.StatusOK:
			j, err := simplejson.NewFromReader(resp.Body)
			if err != nil {
				return nil, err
			}
			jLen := len(j.MustArray())
			klines := make([]*models.Kline, jLen)
			for ind := 0; ind < jLen; ind++ {
				item := j.GetIndex(ind)
				klines[ind] = &models.Kline{
					OpenTime:                 time.Unix(item.GetIndex(0).MustInt64(), 0),
					Open:                     item.GetIndex(1).MustFloat64(),
					High:                     item.GetIndex(2).MustFloat64(),
					Low:                      item.GetIndex(3).MustFloat64(),
					Close:                    item.GetIndex(4).MustFloat64(),
					Volume:                   item.GetIndex(5).MustFloat64(),
					CloseTime:                time.Unix(item.GetIndex(6).MustInt64(), 0),
					QuoteAssetVolume:         item.GetIndex(7).MustFloat64(),
					TradeNum:                 item.GetIndex(8).MustInt64(),
					TakerBuyBaseAssetVolume:  item.GetIndex(9).MustFloat64(),
					TakerBuyQuoteAssetVolume: item.GetIndex(10).MustFloat64(),
				}
			}
			return klines, nil
		case 418, 429:
			waitCount, err := strconv.ParseUint(
				resp.Header.Get("Retry-After"), 10, 64,
			)
			if err != nil {
				err = nil
				waitCount = 10
			}
			me.Logger.Warn(
				"Got locked out!! Waiting...",
				zap.Int("status", resp.StatusCode),
				zap.Uint64("duration", waitCount),
			)
			<-time.After(time.Duration(waitCount) * time.Second)
			break
		case http.StatusNotFound:
			return nil, nil
		default:
			dec := json.NewDecoder(resp.Body)
			apiErr := &common.APIError{}
			if decErr := dec.Decode(apiErr); decErr != nil {
				return nil, decErr
			}
			return nil, apiErr
		}
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
	endTime := time.Now().UTC().Add(time.Minute)
	startTime := endTime.Add(-999 * time.Minute)
	for _, pair := range targetSymbols {
		done := true
		for done {
			findCtx, stop := context.WithTimeout(ctx, 10*time.Second)
			defer stop()
			cur, err := me.Col.Find(findCtx, bson.M{
				"symbol": pair,
				"opentime": bson.M{
					"$gte": startTime,
					"$lte": endTime,
				},
			})
			if err != nil {
				// start fetch since startTime until endTime
			}
			endTime = startTime.Add(-1 * time.Minute)
			startTime = endTime.Add(-999 * time.Minute)
		}
	}
	return nil
}
