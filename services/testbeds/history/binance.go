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
	query.Set("endTime", strconv.FormatInt(endAt.Unix()*1000, 10))
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
	me.Logger.Info("Pair to fetch", zap.Any("pairs", targetSymbols))
	endAt := time.Now().UTC().Add(-1 * time.Minute)
	endAt = endAt.Add(
		-time.Duration(endAt.Second())*time.Second -
			time.Duration(endAt.Nanosecond())*time.Nanosecond,
	)
	startAt := endAt.Add(-999 * time.Minute)
	for _, pair := range targetSymbols {
		for {
			findCtx, stop := context.WithTimeout(ctx, 10*time.Second)
			defer stop()
			cur, err := me.Col.Find(findCtx, bson.M{
				"symbol": pair,
				"openat": bson.M{
					"$gte": startAt,
					"$lt":  endAt,
				},
				"closeat": bson.M{
					"$gt":  startAt,
					"$lte": endAt,
				},
			}, options.Find().SetSort(bson.M{
				"closeat": -1,
			}))
			var klines []*models.Kline
			if err != nil {
				var err error
				klines, err = me.fetch(pair, startAt, endAt)
				if err != nil {
					return err
				}
				me.Logger.Info(
					"Fetched k lines data",
					zap.Time("since", startAt),
					zap.Time("until", endAt),
				)
			} else {
				nextCtx, stopNext := context.WithTimeout(ctx, 10*time.Second)
				defer stopNext()
				partialEndAt := endAt
				for cur.Next(nextCtx) {
					kline := &models.Kline{}
					cur.Decode(kline)
					if kline.CloseAt != partialEndAt.Add(-1*time.Millisecond) ||
						kline.OpenAt != partialEndAt.Add(-1*time.Minute) {
						break
					}
					me.Logger.Info(
						"Cache Hit",
						zap.Time("start", startAt),
						zap.Time("end", partialEndAt),
					)
					partialEndAt = partialEndAt.Add(-1 * time.Minute)
				}
				if partialEndAt.Add(-1*time.Minute) != startAt {
					delCtx, stopDel := context.WithTimeout(ctx, 10*time.Second)
					defer stopDel()
					_, err := me.Col.DeleteMany(delCtx, bson.M{
						"closeat": bson.M{
							"$gt": startAt,
							"$lt": partialEndAt,
						},
					})
					if err != nil {
						return err
					}
					klines, err = me.fetch(pair, startAt, partialEndAt)
					if err != nil {
						return err
					}
					me.Logger.Info(
						"Fetched k lines data",
						zap.Time("since", startAt),
						zap.Time("until", partialEndAt),
					)
				}
			}
			if klines == nil || len(klines) < 1 {
				break
			}
			toInsert := make([]interface{}, len(klines))
			for ind, item := range klines {
				toInsert[ind] = item
			}
			insCtx, stopIns := context.WithTimeout(ctx, 10*time.Second)
			defer stopIns()
			_, err = me.Col.InsertMany(insCtx, toInsert)
			if err != nil {
				return err
			}
			endAt = startAt
			startAt = endAt.Add(-999 * time.Minute)
		}
	}
	return nil
}
