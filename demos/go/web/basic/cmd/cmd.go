package cmd

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"
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
	// @deprecated use profile instead
	// port := flag.Int("port", defaultPort, "port number")
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s [-port port_number]\n", os.Args[0])
		flag.PrintDefaults()
	}
	flag.Parse()
	conf, err := profile.GetProfile(appName)
	if err != nil {
		log.Fatalf("failed to get profile, error: %+v\n", err)
	}
	initConfig(conf, config)
	// conf.Port = *port
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
