package services

import (
	"web/basic-orm/model"
	"web/basic-orm/store"
	"web/common/sqls"
)

type userService struct{}

var UserService = &userService{}

func (s *userService) Create(t *model.User) error {
	return store.Db().Create(t).Error
	// if err == nil {
	// 	cache.UserCache.Invalidate(t.Id)
	// }
}

func (s *userService) Get(id int64) *model.User {
	user := &model.User{}
	if err := store.Db().First(user, "id = ?", id).Error; err != nil {
		return nil
	}
	return user
}

func (s *userService) Find(cnd *sqls.Cnd) []model.User {
	list := make([]model.User, 0)
	cnd.Find(store.Db(), &list)
	return list
}
