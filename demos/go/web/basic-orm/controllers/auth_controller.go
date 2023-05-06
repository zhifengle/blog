package controllers

import (
	"web/basic-orm/model"
	"web/basic-orm/model/api"
	"web/basic-orm/services"
	"web/common/web"

	"github.com/gin-gonic/gin"
)

type AuthController struct {
}

func (c *AuthController) Register(ctx *gin.Context) {
	userCreate := &api.UserCreate{}
	if err := ctx.ShouldBindJSON(userCreate); err != nil {
		ctx.JSON(200, web.JsonErrorCode(web.ERROR_INVALID_PARAM))
		return
	}
	if err := userCreate.Validate(); err != nil {
		ctx.JSON(200, web.JsonErrorCode(web.ERROR_INVALID_PARAM))
		return
	}
	user := &model.User{
		Username: userCreate.Username,
		Password: userCreate.Password,
		Email:    userCreate.Email,
		Nickname: userCreate.Nickname,
	}
	err := services.UserService.Create(user)
	if err != nil {
		ctx.JSON(200, web.JsonError(err))
		return
	}
	ctx.JSON(200, web.JsonSuccess())
}
