package sqls

import (
	"time"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
	"gorm.io/gorm/schema"
)

type DbConfig struct {
	DSN                    string `json:"DSN"`
	MaxIdleConns           int    `json:"MaxIdleConns"`
	MaxOpenConns           int    `json:"MaxOpenConns"`
	ConnMaxIdleTimeSeconds int    `json:"ConnMaxIdleTimeSeconds"`
	ConnMaxLifetimeSeconds int    `json:"ConnMaxLifetimeSeconds"`
}

func Open(dbConfig DbConfig, config *gorm.Config, models ...interface{}) (*gorm.DB, error) {
	if config == nil {
		config = &gorm.Config{}
	}

	if config.NamingStrategy == nil {
		config.NamingStrategy = schema.NamingStrategy{
			TablePrefix:   "t_",
			SingularTable: true,
		}
	}
	db, err := gorm.Open(mysql.Open(dbConfig.DSN), config)
	if err != nil {
		return nil, err
	}

	if sqlDB, err := db.DB(); err == nil {
		sqlDB.SetMaxIdleConns(dbConfig.MaxIdleConns)
		sqlDB.SetMaxOpenConns(dbConfig.MaxOpenConns)
		sqlDB.SetConnMaxIdleTime(time.Duration(dbConfig.ConnMaxIdleTimeSeconds) * time.Second)
		sqlDB.SetConnMaxLifetime(time.Duration(dbConfig.ConnMaxLifetimeSeconds) * time.Second)
	}

	err = db.AutoMigrate(models...)
	if err != nil {
		return nil, err
	}
	return db, nil
}
