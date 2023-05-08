package services

import (
	"web/basic-orm/model/api"

	"gorm.io/gorm"
)

func selectPages[T any](db *gorm.DB, p *api.PageResult[T]) (e error) {
	var model T
	db.Model(&model).Count(&p.Total)
	if p.Total == 0 {
		p.Data = []T{}
		return
	}
	if p.CurrentPage == 0 {
		p.CurrentPage = 1
	}
	offset := (p.CurrentPage - 1) * p.PageSize
	switch {
	case p.PageSize > 10000:
		p.PageSize = 10000
	case p.PageSize <= 0:
		p.PageSize = 10
	}
	e = db.Model(&model).Offset(offset).Limit(p.PageSize).Find(&p.Data).Error
	return
}
