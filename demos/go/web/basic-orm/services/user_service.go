package services

import (
	"web/basic-orm/model"
	"web/basic-orm/model/api"
	"web/basic-orm/store"
	"web/common/web"
)

type userService struct{}

var UserService = &userService{}

func (s *userService) Create(t *model.User) error {
	user := s.GetByUsername(t.Username)
	if user != nil {
		return web.NewCodeError(web.CODE_USER_NAME_USED)
	}
	err := store.Db().Create(t).Error
	return err
}

func (s *userService) Get(id int64) *model.User {
	user := &model.User{}
	if err := store.Db().First(user, "id = ?", id).Error; err != nil {
		return nil
	}
	return user
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

func (s *userService) GetList(paging *api.Paging) (*api.PageResult[model.User], error) {
	p := &api.PageResult[model.User]{
		PageSize:    paging.PageSize,
		CurrentPage: paging.CurrentPage,
	}
	err := selectPages(store.Db(), p)
	return p, err
}
