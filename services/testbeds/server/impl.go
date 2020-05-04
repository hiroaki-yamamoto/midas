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
		trade := &models.Trade{}
		if err := cur.Decode(trade); err != nil {
			return err
		}
		ts, err := ptypes.TimestampProto(trade.Timestamp)
		if err != nil {
			return err
		}
		ret := &rpc.SubscribeResponse{
			Symbol:    trade.Symbol,
			Timestamp: ts,
			Price:     trade.Price,
			Qty:       trade.Qty,
		}
		switch trade.Type {
		case models.BUY:
			ret.Type = rpc.TradeType_Buy
			break
		case models.SELL:
			ret.Type = rpc.TradeType_Sell
			break
		}
		if err := resp.SendMsg(ret); err != nil {
			return err
		}
	}
	return nil
}
