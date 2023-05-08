package api

type Paging struct {
	CurrentPage int `json:"currentPage"`
	PageSize    int `json:"pageSize"`
}

type PageResult[T any] struct {
	CurrentPage int   `json:"currentPage"`
	PageSize    int   `json:"pageSize"`
	Total       int64 `json:"total"`
	Data        []T   `json:"data"`
}
