package cmd

import (
	"context"
	"flag"
	"log"
	"web/basic-orm/config"
	"web/basic-orm/model"
	"web/basic-orm/server"
	"web/basic-orm/store"
	"web/basic-orm/store/db"
	"web/common"

	"gorm.io/gorm"
)

func Execute() {
	var configFile string
	flag.StringVar(&configFile, "c", "config.yaml", "config file")
	flag.Parse()
	cfg := config.Init(configFile)

	database, err := db.Open(cfg.DB, &gorm.Config{}, model.Models...)
	if err != nil {
		log.Fatalf("failed to open db, error: %+v\n", err)
	}
	s, err := server.NewServer(context.Background(), &cfg.Server, store.Init(database))
	if err != nil {
		log.Fatalf("failed to create server, error: %+v\n", err)
	}
	common.StartServer(s)
}
