package config

import (
	"log"
	"web/basic-orm/server/profile"
	"web/common/sqls"

	"github.com/spf13/viper"
)

var Instance = &Config{}

const (
	appName     = "basic-orm"
	defaultPort = 7080
)

type Config struct {
	Server profile.Profile `json:"server"`

	DB sqls.DbConfig `json:"db"`

	Jwt struct {
		SignKey       string `json:"signKey"`
		ExpireSeconds int    `json:"expireSeconds"`
		Issuer        string `json:"issuer"`
	} `json:"jwt"`
}

func Init(configFile string) *Config {
	viper.SetDefault("port", defaultPort)
	viper.SetEnvPrefix(appName)

	v := viper.New()
	v.SetConfigFile(configFile)
	v.SetConfigType("yaml")
	err := v.ReadInConfig()
	if err != nil {
		log.Fatalf("Fatal error config file: %s \n", err)
	}
	if err = v.Unmarshal(Instance); err != nil {
		log.Fatalf("invalid config file: %s \n", err)
	}
	return Instance
}
