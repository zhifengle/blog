package services

import (
	"web/basic-orm/model"
	"web/basic-orm/store"
)

type userService struct{}

var UserService = &userService{}
var db = store.Db()

func (s *userService) Get(id int64) *model.User {
	user := &model.User{}
	if err := db.First(user, "id = ?", id).Error; err != nil {
		return nil
	}
	return user
}

func (s *userService) Find(cnd *store.Cnd) []model.User {
	list := make([]model.User, 0)
	cnd.Find(db, &list)
	return list
}
