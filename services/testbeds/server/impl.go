package server

import (
	"github.com/golang/protobuf/ptypes"
	"github.com/hiroaki-yamamoto/midas/services/testbeds/models"
	"github.com/hiroaki-yamamoto/midas/services/testbeds/rpc"
	"go.mongodb.org/mongo-driver/bson"
)

// Subscribe reads the trade data.
func (me *Server) Subscribe(
	req *rpc.SubscribeRequest,
	resp rpc.TestBed_SubscribeServer,
) error {
	cur, err := me.Col.Find(resp.Context(), bson.M{
		"symbol": req.GetSymbol(),
	})
	if err != nil {
		return err
	}
	for cur.Next(resp.Context()) {
		trade := &models.Kline{}
		if err := cur.Decode(trade); err != nil {
			return err
		}
		ts, err := ptypes.TimestampProto(trade.OpenAt)
		if err != nil {
			return err
		}
		ret := &rpc.SubscribeResponse{
			Symbol:    trade.Symbol,
			Timestamp: ts,
			Qty:       trade.QuoteAssetVolume,
		}
		switch req.GetPriceBase() {
		case rpc.PriceBase_High:
			ret.Price = trade.High
			break
		case rpc.PriceBase_Low:
			ret.Price = trade.Low
			break
		case rpc.PriceBase_MidHighAndLow:
			ret.Price = (trade.High + trade.Low) / 2
		case rpc.PriceBase_Open:
			ret.Price = trade.Open
			break
		case rpc.PriceBase_Close:
			ret.Price = trade.Close
			break
		case rpc.PriceBase_MidOpenClose:
			ret.Price = (trade.Open + trade.Close) / 2
		}
		if err := resp.SendMsg(ret); err != nil {
			return err
		}
	}
	return nil
}
