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

func (s *userService) GetByUsername(username string) *model.User {
	return s.Take("username = ?", username)
}

func (s *userService) Take(where ...interface{}) *model.User {
	ret := &model.User{}
	if err := store.Db().Take(ret, where...).Error; err != nil {
		return nil
	}
	return ret
}
