package store

import (
	"database/sql"
	"web/basic/server/profile"
)

type Store struct {
	db      *sql.DB
	profile *profile.Profile
}

func New(db *sql.DB, cfg *profile.Profile) *Store {
	return &Store{
		db:      db,
		profile: cfg,
	}
}
