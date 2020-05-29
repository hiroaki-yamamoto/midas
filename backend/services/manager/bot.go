package manager

import (
	"context"
	"time"

	"github.com/hiroaki-yamamoto/midas/backend/rpc"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

// BotManager represents the bot management server
type BotManager struct {
	Col *mongo.Collection
}

// NewBotManager constructs a new bot manager.
func (me *BotManager) NewBotManager(col *mongo.Collection) *BotManager {
	return &BotManager{Col: col}
}

// ListBotInfo lists the bot information.
func (me *BotManager) ListBotInfo(
	ctx context.Context,
	req *rpc.BotInfoListRequest,
) (*rpc.BotInfoList, error) {
	tCtx, stop := context.WithTimeout(ctx, 10*time.Second)
	defer stop()
	cur, err := me.Col.Find(
		tCtx, bson.M{},
		options.Find().SetSkip(req.GetOffset()).SetLimit(req.GetLimit()),
	)
	if err != nil {
		return nil, err
	}
	var botInfo []*rpc.BotInfo
	if err := cur.All(tCtx, &botInfo); err != nil {
		return nil, err
	}
	return &rpc.BotInfoList{Bots: botInfo}, nil
}
