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
	"go.mongodb.org/mongo-driver/mongo/options"
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
	startAt, endAt time.Time,
) ([]*models.Kline, error) {
	const endpoint = "https://api.binance.com/api/v3/klines"
	query := make(url.Values)
	query.Set("symbol", pair)
	query.Set("interval", "1m")
	query.Set("startTime", strconv.FormatInt(startAt.Unix()*1000, 10))
	if !endAt.IsZero() {
		query.Set("endTime", strconv.FormatInt(endAt.Unix()*1000, 10))
	}
	timeDiff := int64(endAt.Sub(startAt) / time.Minute)
	if timeDiff > 0 {
		query.Set("limit", "1000")
	} else {
		query.Set("limit", "1")
	}
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
				var open, close, high, low, vol float64
				var QuoteAssetVolume, TakerBuyBaseAssetVolume, TakerBuyQuoteAssetVolume float64
				var err error
				if open, err = strconv.ParseFloat(item.GetIndex(1).MustString(), 64); err != nil {
					return nil, err
				}
				if close, err = strconv.ParseFloat(item.GetIndex(4).MustString(), 64); err != nil {
					return nil, err
				}
				if high, err = strconv.ParseFloat(item.GetIndex(2).MustString(), 64); err != nil {
					return nil, err
				}
				if low, err = strconv.ParseFloat(item.GetIndex(3).MustString(), 64); err != nil {
					return nil, err
				}
				if vol, err = strconv.ParseFloat(item.GetIndex(5).MustString(), 64); err != nil {
					return nil, err
				}
				if QuoteAssetVolume, err = strconv.ParseFloat(
					item.GetIndex(7).MustString(), 64,
				); err != nil {
					return nil, err
				}
				if TakerBuyBaseAssetVolume, err = strconv.ParseFloat(
					item.GetIndex(9).MustString(), 64,
				); err != nil {
					return nil, err
				}
				if TakerBuyQuoteAssetVolume, err = strconv.ParseFloat(
					item.GetIndex(10).MustString(), 64,
				); err != nil {
					return nil, err
				}
				klines[ind] = &models.Kline{
					Symbol: pair,
					OpenAt: time.Unix(
						item.GetIndex(0).MustInt64()/1000,
						int64(
							time.Duration(item.GetIndex(0).MustInt64()%1000)*time.Millisecond,
						),
					),
					Open:   open,
					High:   high,
					Low:    low,
					Close:  close,
					Volume: vol,
					CloseAt: time.Unix(
						item.GetIndex(6).MustInt64()/1000,
						int64(
							time.Duration(item.GetIndex(6).MustInt64()%1000)*time.Millisecond,
						),
					),
					QuoteAssetVolume:         QuoteAssetVolume,
					TradeNum:                 item.GetIndex(8).MustInt64(),
					TakerBuyBaseAssetVolume:  TakerBuyBaseAssetVolume,
					TakerBuyQuoteAssetVolume: TakerBuyQuoteAssetVolume,
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
			me.Logger.Warn("The response status code got NotFound...")
			return nil, nil
		default:
			me.Logger.Warn(
				"Got irregular status code.", zap.Int("code", resp.StatusCode),
			)
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
	me.Logger.Info("Pair to fetch", zap.Any("pairs", targetSymbols))
	endAt := time.Now().UTC()
	endAt = endAt.Add(
		-time.Duration(endAt.Second())*time.Second -
			time.Duration(endAt.Nanosecond())*time.Nanosecond,
	)
	startAt := endAt.Add(-1000 * time.Minute)
	for _, pair := range targetSymbols {
		me.Logger.Info("Fetching pair...", zap.Any("pair", pair))
		firstKlines, err := me.fetch(pair, time.Time{}, time.Time{})
		if err != nil {
			me.Logger.Error("Error on fetching first kline date", zap.Error(err))
		}
		firstKline := firstKlines[0]
		firstEndAt := endAt
		firstStartAt := startAt
		for (startAt.After(firstKline.OpenAt) || startAt.Equal(firstKline.OpenAt)) ||
			(endAt.After(firstKline.CloseAt) || endAt.Equal(firstKline.CloseAt)) {
			dbCtx, stop := context.WithTimeout(ctx, 10*time.Second)
			defer stop()
			klines, err := func() ([]*models.Kline, error) {
				defer func() {
					endAt = startAt
					startAt = endAt.Add(-1000 * time.Minute)
				}()
				cur, err := me.Col.Find(dbCtx, bson.M{
					"symbol": pair,
					"openat": bson.M{
						"$gte": startAt,
					},
					"closeat": bson.M{
						"$lte": endAt,
					},
				}, options.Find().SetSort(bson.M{
					"closeat": -1,
				}).SetLimit(1))
				var klines []*models.Kline
				if err != nil {
					klines, err = me.fetch(pair, startAt, endAt)
				} else {
					if cur.Next(dbCtx) {
						kline := &models.Kline{}
						cur.Decode(kline)
						startAt = kline.CloseAt.Add(1 * time.Millisecond)
					}
					if !startAt.Before(endAt) {
						return nil, nil
					}
					klines, err = me.fetch(pair, startAt, endAt)
				}
				return klines, err
			}()
			if err != nil {
				me.Logger.Error("Error while fetching", zap.Error(err))
				continue
			}
			if klines == nil || len(klines) < 1 {
				continue
			}
			toInsert := make([]interface{}, len(klines))
			for ind, item := range klines {
				toInsert[ind] = item
			}
			_, err = me.Col.InsertMany(dbCtx, toInsert)
			if err != nil {
				me.Logger.Error("Error while inseting data to ", zap.Error(err))
			}
			me.Logger.Info(
				"Fetched k lines data",
				zap.Time("start", startAt),
				zap.Time("end", endAt),
			)
		}
		startAt, endAt = firstStartAt, firstEndAt
	}
	return nil
}
