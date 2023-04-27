package cmd

import (
	"context"
	"flag"
	"log"
	"web/basic/server"
	"web/basic/server/profile"
	"web/common"

	"github.com/spf13/viper"
)

const (
	appName     = "basic"
	defaultPort = 7080
)

func Execute() {
	var config string
	flag.StringVar(&config, "c", "config.yaml", "config file")
	flag.Parse()
	conf, err := profile.GetProfile(appName)
	if err != nil {
		log.Fatalf("failed to get profile, error: %+v\n", err)
	}
	initConfig(conf, config)
	s, err := server.NewServer(context.Background(), conf)
	if err != nil {
		log.Fatalf("failed to create server, error: %+v\n", err)
	}
	common.StartServer(s)
}

func initConfig(profile *profile.Profile, config string) {
	viper.SetDefault("port", defaultPort)
	viper.SetEnvPrefix(appName)

	v := viper.New()
	v.SetConfigFile(config)
	v.SetConfigType("yaml")
	err := v.ReadInConfig()
	if err != nil {
		log.Fatalf("Fatal error config file: %s \n", err)
	}
	if err = v.Unmarshal(profile); err != nil {
		log.Fatalf("invalid config file: %s \n", err)
	}

}
