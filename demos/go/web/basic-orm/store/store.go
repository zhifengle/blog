package store

import (
	"gorm.io/gorm"
)

var dbIns *gorm.DB

type Store struct {
}

func Init(db *gorm.DB) *Store {
	dbIns = db
	return &Store{}
}

func Db() *gorm.DB {
	return dbIns
}

func (s *Store) Close() error {
	if dbIns == nil {
		return nil
	}
	sqlDB, err := dbIns.DB()
	if err == nil {
		return err
	}
	return sqlDB.Close()
}
