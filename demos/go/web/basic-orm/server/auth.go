package server

import (
	"web/basic-orm/model"
	"web/basic-orm/model/api"
	"web/basic-orm/services"
	"web/common/dates"
	"web/common/web"

	"github.com/gin-gonic/gin"
)

func register(ctx *gin.Context) {
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
		Username:   userCreate.Username,
		Password:   userCreate.Password,
		Email:      userCreate.Email,
		Nickname:   userCreate.Nickname,
		Status:     0,
		CreateTime: dates.NowTimestamp(),
		UpdateTime: dates.NowTimestamp(),
	}
	err := services.UserService.Create(user)
	if err != nil {
		ctx.JSON(200, web.JsonError(err))
		return
	}
	ctx.JSON(200, web.JsonSuccess())
}

func signin(c *gin.Context) {
}

func (s *Server) registerAuthRoutes(g *gin.RouterGroup, secret string) {
	g.POST("/auth/signin", signin)

	g.POST("/auth/register", register)
}
