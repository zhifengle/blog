package db

import (
	"context"
	"database/sql"
	"embed"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"sort"
	"web/basic/server/profile"

	_ "github.com/mattn/go-sqlite3"
)

//go:embed migration
var migrationFS embed.FS

//go:embed seed
var seedFS embed.FS

const (
	latestSchemaFileName = "LATEST__SCHEMA.sql"
)

type DB struct {
	// sqlite db connection instance
	DBInstance *sql.DB
	profile    *profile.Profile
}

func New(profile *profile.Profile, ctx context.Context) (*DB, error) {
	if profile.DSN == "" {
		return nil, fmt.Errorf("dsn required")
	}
	sqliteDB, err := sql.Open("sqlite3", profile.DSN+"?cache=shared&_foreign_keys=0&_journal_mode=WAL")
	if err != nil {
		return nil, fmt.Errorf("failed to open db with dsn: %s, err: %w", profile.DSN, err)
	}
	db := &DB{
		DBInstance: sqliteDB,
		profile:    profile,
	}
	if _, err := os.Stat(profile.DSN); errors.Is(err, os.ErrNotExist) {
		if err := db.applyLatestSchema(ctx); err != nil {
			return nil, fmt.Errorf("failed to apply latest schema: %w", err)
		}
		// In demo mode, we should seed the database.
		if profile.Mode == "demo" {
			if err := db.seed(ctx); err != nil {
				return nil, fmt.Errorf("failed to seed: %w", err)
			}
		}
	}

	return db, nil
}

func (db *DB) applyLatestSchema(ctx context.Context) error {
	schemaMode := "dev"
	if db.profile.Mode == "prod" {
		schemaMode = "prod"
	}
	latestSchemaPath := fmt.Sprintf("%s/%s/%s", "migration", schemaMode, latestSchemaFileName)
	buf, err := migrationFS.ReadFile(latestSchemaPath)
	if err != nil {
		return fmt.Errorf("failed to read latest schema %q, error %w", latestSchemaPath, err)
	}
	stmt := string(buf)
	if err := db.execute(ctx, stmt); err != nil {
		return fmt.Errorf("migrate error: statement:%s err=%w", stmt, err)
	}
	return nil
}

func (db *DB) seed(ctx context.Context) error {
	filenames, err := fs.Glob(seedFS, fmt.Sprintf("%s/*.sql", "seed"))
	if err != nil {
		return fmt.Errorf("failed to read seed files, err: %w", err)
	}

	sort.Strings(filenames)

	// Loop over all seed files and execute them in order.
	for _, filename := range filenames {
		buf, err := seedFS.ReadFile(filename)
		if err != nil {
			return fmt.Errorf("failed to read seed file, filename=%s err=%w", filename, err)
		}
		stmt := string(buf)
		if err := db.execute(ctx, stmt); err != nil {
			return fmt.Errorf("seed error: statement:%s err=%w", stmt, err)
		}
	}
	return nil
}

func (db *DB) execute(ctx context.Context, stmt string) error {
	tx, err := db.DBInstance.Begin()
	if err != nil {
		return err
	}
	defer tx.Rollback()

	if _, err := tx.ExecContext(ctx, stmt); err != nil {
		return fmt.Errorf("failed to execute statement, err: %w", err)
	}

	return tx.Commit()
}
