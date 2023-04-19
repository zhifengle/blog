package cmd

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"
	"web/auth/server"
	"web/auth/server/config"
	"web/common"
)

func Execute() {
	port := flag.Int("port", 7080, "port number")
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s [-port port_number]\n", os.Args[0])
		flag.PrintDefaults()
	}
	flag.Parse()
	conf := &config.Config{
		Port:    *port,
		Version: "v0.1.0",
	}
	s, err := server.NewServer(context.Background(), conf)
	if err != nil {
		log.Fatalf("failed to create server, error: %+v\n", err)
	}
	common.StartServer(s)
}
