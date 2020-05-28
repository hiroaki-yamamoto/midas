package main

import (
	"context"
	"fmt"
	"log"
	"sync"
	"time"

	"github.com/adshao/go-binance"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

var logger *zap.Logger

func init() {
	var err error
	cfg := zap.NewDevelopmentConfig()
	cfg.EncoderConfig.EncodeLevel = zapcore.CapitalColorLevelEncoder
	logger, err = cfg.Build()
	if err != nil {
		logger = zap.NewExample()
		log.Println(
			"[WARN]: Initialization Loggger Failed. Using Example Logger.",
			err,
		)
	}
}

func tradeHandler(event *binance.WsTradeEvent) {
	typeTxt := "🔥 Buy"
	if event.IsBuyerMaker {
		typeTxt = "💰 Sell"
	}
	logger.Info(fmt.Sprintf("Trade(%s): ", typeTxt), zap.Any("event", event))
}

func main() {
	rootCtx := context.Background()
	cli := binance.NewClient("", "")
	infoReqCtx, cancel := context.WithTimeout(rootCtx, 10*time.Second)
	defer cancel()
	excInfoSrv := cli.NewExchangeInfoService()
	excInfo, err := excInfoSrv.Do(infoReqCtx)
	if err != nil {
		logger.Panic("Error while reading symbol info:", zap.Error(err))
	}
	var wg sync.WaitGroup
	wg.Add(len(excInfo.Symbols))
	for _, sym := range excInfo.Symbols {
		go func(sym string) {
			defer wg.Done()
			logger.Info(fmt.Sprintf("Connect: %s", sym))
			doneC, _, err := binance.WsTradeServe(
				sym, tradeHandler, func(err error) {
					logger.Panic(
						"Error while reading payload from the server:", zap.Error(err),
					)
				},
			)
			if err != nil {
				logger.Panic("Error while connecting to the server:", zap.Error(err))
			}
			<-doneC
		}(sym.Symbol)
	}
	wg.Wait()
}
