package store

import (
	"context"
	"database/sql"
	"sync"
	"web/basic/server/profile"
)

type Store struct {
	db               *sql.DB
	profile          *profile.Profile
	userCache        sync.Map // map[int]*userRaw
	userSettingCache sync.Map // map[string]*userSettingRaw
}

func New(db *sql.DB, cfg *profile.Profile) *Store {
	return &Store{
		db:      db,
		profile: cfg,
	}
}

func vacuum(ctx context.Context, tx *sql.Tx) error {
	if err := vacuumUserSetting(ctx, tx); err != nil {
		return err
	}

	return nil
}

func (s *Store) Close() error {
	return s.db.Close()
}
