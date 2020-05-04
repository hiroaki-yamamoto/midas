package main

import (
	stdlog "log"
	"os"

	"github.com/mkideal/cli"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

var log *zap.Logger
var cmdRoot *cli.Command

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
	Fn: func(ctx *cli.Context) error {
		return nil
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
	)
}

func main() {
	if err := cmdRoot.Run(os.Args[1:]); err != nil {
		log.Fatal("Commandline parse error", zap.Error(err))
		panic("")
	}
}
