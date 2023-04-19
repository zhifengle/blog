package store

import (
	"database/sql"
	"fmt"
)

type bookRaw struct {
	ID     int
	Name   string
	Author string
}

func (b *bookRaw) toBook() *Book {
	return &Book{
		ID:     b.ID,
		Name:   b.Name,
		Author: b.Author,
	}
}

type Book struct {
	ID     int    `json:"id"`
	Name   string `json:"name"`
	Author string `json:"author"`
}

type BookCreate struct {
	Name   string `json:"name"`
	Author string `json:"author"`
}

func (b *BookCreate) Validate() error {
	if len(b.Name) == 0 {
		return fmt.Errorf("name is empty")
	}
	return nil
}

func (s *Store) CreateBook(book *BookCreate) (*Book, error) {
	tx, err := s.db.Begin()
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()
	bookRaw, err := createBook(tx, book)
	if err != nil {
		return nil, err
	}
	if err := tx.Commit(); err != nil {
		return nil, err
	}
	return bookRaw.toBook(), nil
}

func createBook(tx *sql.Tx, book *BookCreate) (*bookRaw, error) {
	query := `
	INSERT INTO books (name, author)
	VALUES (?, ?)
	RETURNING id, name, author
	`
	var bookRaw bookRaw
	err := tx.QueryRow(query, book.Name, book.Author).Scan(&bookRaw.ID, &bookRaw.Name, &bookRaw.Author)
	if err != nil {
		return nil, err
	}
	return &bookRaw, nil
}
