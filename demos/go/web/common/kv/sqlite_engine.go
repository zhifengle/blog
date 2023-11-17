package kv

import (
	"database/sql"
)

type SqliteEngine struct {
	db *sql.DB
}

func NewSqliteEngine(db *sql.DB) *SqliteEngine {
	// db.Exec("CREATE TABLE if not exists `kv_items` `id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `value` TEXT, `createdAt` DATETIME NOT NULL, `updatedAt` DATETIME NOT NULL)")
	db.Exec("CREATE TABLE if not exists `kv_items` (`name` VARCHAR(2047), `value` TEXT)")
	return &SqliteEngine{
		db: db,
	}
}

func (s *SqliteEngine) set(key string, val interface{}) {
	// s.db.Exec("INSERT INTO kv_items (`name`,`value`,`createdAt`,`updatedAt`) VALUES (?,?,?,?)", key, val, time.Now(), time.Now())
	s.db.Exec("INSERT INTO kv_items (`name`,`value`) VALUES (?,?)", key, val)
}

func (s *SqliteEngine) get(key string) interface{} {
	var val any
	s.db.QueryRow("SELECT value FROM kv_items WHERE name = ?", key).Scan(&val)
	return val
}

func (s *SqliteEngine) remove(key string) {
	s.db.Exec("DELETE FROM kv_items WHERE name = ?", key)
}

func (s *SqliteEngine) keys() []string {
	var keys []string
	rows, _ := s.db.Query("SELECT name FROM kv_items")
	for rows.Next() {
		var key string
		rows.Scan(&key)
		keys = append(keys, key)
	}
	return keys
}
