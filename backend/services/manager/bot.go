package manager

import "go.mongodb.org/mongo-driver/mongo"

// BotManager represents the bot management server
type BotManager struct {
	Col *mongo.Collection
}

// NewBotManager constructs a new bot manager.
func (me *BotManager) NewBotManager(col *mongo.Collection) *BotManager {
	return &BotManager{Col: col}
}
