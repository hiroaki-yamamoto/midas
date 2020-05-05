package main

import (
	"context"
	"fmt"
	stdlog "log"
	"net"
	"os"
	"syscall"
	"time"

	"github.com/go-playground/validator/v10"
	"github.com/hiroaki-yamamoto/midas/services/testbeds/server"
	"github.com/mkideal/cli"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

var log *zap.Logger
var cmdRoot *cli.Command
var vld = validator.New()

type rootMenu struct {
	Help bool `cli:"h,help" dft:"true" usage:"Display Help Information."`
}

func (me *rootMenu) AutoHelp() bool {
	return me.Help
}

var root = &cli.Command{
	Desc: "Binance Testbed Server",
	Argv: func() interface{} {
		return &rootMenu{}
	},
	Fn: func(ctx *cli.Context) error { return nil },
}
var helpTree = cli.HelpCommand("Display Help Information.")

type downloadMenu struct {
	cli.Helper
	Symbol string `cli:"s,symbol" dft:"all" usage:"Download the historical data of the specified specified symbol."`
}

var download = &cli.Command{
	Name: "get",
	Desc: "Download 1 minute-ticked historical data from binance.",
	Argv: func() interface{} { return &downloadMenu{} },
	Fn:   func(ctx *cli.Context) error { return nil },
}

type svrMenu struct {
	cli.Helper
	Exchange    string `cli:"*e,exchange" usage:"Set exchange to test" validate:"required,oneof=binance"`
	NetworkType string `cli:"*n,netowrk.type" usage:"Set the type of the network." validate:"required"`
	NetworkAddr string `cli:"*a,netowrk.addr" usage:"Set the address to bind."`
	DBURL       string `cli:"*d,dbURLa" usage:"Datanase URL."`
}

func (me *svrMenu) Validate(ctx *cli.Context) error {
	return vld.Struct(me)
}

var serverCmd = &cli.Command{
	Name: "run",
	Desc: "Start the server",
	Argv: func() interface{} { return &svrMenu{} },
	Fn: func(ctx *cli.Context) error {
		conCtx, stop := context.WithTimeout(context.Background(), 10*time.Second)
		defer stop()
		menu := ctx.Argv().(*svrMenu)
		con, err := mongo.Connect(conCtx, options.Client().ApplyURI(menu.DBURL))
		if err != nil {
			return err
		}
		db := con.Database("midas-testbed")
		col := db.Collection(menu.Exchange)
		lis, err := net.Listen(menu.NetworkType, menu.NetworkAddr)
		if err != nil {
			return err
		}
		svr := server.New(log, lis, col)
		svr.Trap(syscall.SIGINT, syscall.SIGTERM)
		log.Info(fmt.Sprintf(
			"%s Testbed Server Started @ %s", menu.Exchange, lis.Addr().String(),
		))
		return svr.Serve()
	},
}

func init() {
	var err error
	cfg := zap.NewDevelopmentConfig()
	cfg.EncoderConfig.EncodeLevel = zapcore.CapitalColorLevelEncoder
	log, err = cfg.Build()
	if err != nil {
		stdlog.Println(
			"Building Logger failed.",
			"Constructing Example Logger Instead.",
		)
		log = zap.NewExample()
	}
	cmdRoot = cli.Root(
		root,
		cli.Tree(helpTree),
		cli.Tree(download),
		cli.Tree(serverCmd),
	)
}

func main() {
	if err := cmdRoot.Run(os.Args[1:]); err != nil {
		panic(err)
	}
}
