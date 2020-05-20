package server

import (
	"net"
	"os"
	"os/signal"

	"github.com/hiroaki-yamamoto/midas/services/testbeds/rpc"
	"go.mongodb.org/mongo-driver/mongo"
	"go.uber.org/zap"
	"google.golang.org/grpc"
)

// Server repersents a server to publish historical data.
type Server struct {
	*grpc.Server
	Lis net.Listener
	Log *zap.Logger
	Col *mongo.Collection
}

// New creates a new instance of Server.
func New(log *zap.Logger, lis net.Listener, col *mongo.Collection) *Server {
	ret := &Server{
		Server: grpc.NewServer(),
		Log:    log,
		Lis:    lis,
		Col:    col,
	}
	rpc.RegisterTestBedServer(ret.Server, ret)
	return ret
}

// Close tries graceful-close the server.
func (me *Server) Close() error {
	me.GracefulStop()
	return me.Lis.Close()
}

// Trap traps the server for graceful stop.
// Note that this function doesn't block the current operation.
func (me *Server) Trap(sigs ...os.Signal) {
	sig := make(chan os.Signal)
	signal.Notify(sig, sigs...)
	go func() {
		defer close(sig)
		for range sig {
			me.Log.Info("Graceful Stop...")
			me.Close()
		}
	}()
}

// Serve starts the server.
func (me *Server) Serve() error {
	return me.Server.Serve(me.Lis)
}
