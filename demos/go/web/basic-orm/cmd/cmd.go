package cmd

import (
	"context"
	"flag"
	"log"
	"web/basic-orm/config"
	"web/basic-orm/model"
	"web/basic-orm/server"
	"web/basic-orm/store"
	"web/common"
	"web/common/sqls"

	"gorm.io/gorm"
)

func Execute() {
	var configFile string
	flag.StringVar(&configFile, "c", "config.yaml", "config file")
	flag.Parse()
	cfg := config.Init(configFile)

	db, err := sqls.Open(cfg.DB, &gorm.Config{}, model.Models...)
	if err != nil {
		log.Fatalf("failed to open db, error: %+v\n", err)
	}
	s, err := server.NewServer(context.Background(), &cfg.Server, store.Init(db))
	if err != nil {
		log.Fatalf("failed to create server, error: %+v\n", err)
	}
	common.StartServer(s)
}
