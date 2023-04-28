package store

import "gorm.io/gorm"

var db *gorm.DB

type Store struct {
	DB *gorm.DB
}

func New(db *gorm.DB) *Store {
	return &Store{
		DB: db,
	}
}

func Db() *gorm.DB {
	return db
}
