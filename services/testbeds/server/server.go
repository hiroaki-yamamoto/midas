package server

import (
	"go.mongodb.org/mongo-driver/mongo"
	"google.golang.org/grpc"
)

// Server repersents a server to publish historical data.
type Server struct {
	*grpc.Server
	Col *mongo.Collection
}

// New creates a new instance of Server.
func New(col *mongo.Collection) *Server {
	return &Server{
		Server: grpc.NewServer(),
		Col:    col,
	}
}
