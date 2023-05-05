package controllers

import (
	"web/basic-orm/model"
	"web/common/web"

	"github.com/gin-gonic/gin"
)

type AuthController struct {
}

func (c *AuthController) Register(ctx *gin.Context) {
	user := &model.User{}
	if err := ctx.ShouldBindJSON(user); err != nil {
		ctx.JSON(200, web.JsonErrorCode(web.ERROR_INVALID_PARAM))
		return
	}
	// err := services.UserService.Create(user)
	// if err != nil {
	// 	ctx.JSON(200, web.JsonErrorCode(web.user))
	// }
	// userCreate := &api.UserCreate{
	// 	Username: signup.Username,
	// 	Nickname: signup.Username,
	// 	Password: signup.Password,
	// }
}
