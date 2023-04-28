package controllers

import (
	"web/common/web"

	"github.com/gin-gonic/gin"
)

type UserController struct {
}

func (c *UserController) GetCurrent(ctx *gin.Context) {
	// user := services.UserTokenService.GetCurrent(c)
	// if user != nil {
	// 	// return web.JsonData(render.BuildUserProfile(user))
	// }
	// return web.JsonSuccess()
}

func (u *UserController) GetBy(ctx *gin.Context) {
	userId := ctx.Param("userId")
	if userId == "" {
		// invalid param
		ctx.JSON(200, web.JsonErrorCode(web.ERROR_INVALID_PARAM))
		return
	}
	// user := cache.UserCache.Get(userId)
	// if user != nil && user.Status != constants.StatusDeleted {
	// 	// return web.JsonData(render.BuildUserDetail(user))
	// 	return
	// }
	ctx.JSON(200, web.JsonErrorCode(web.ERROR_USER_NOT_EXIST))
}

func (c *UserController) Create(ctx *gin.Context) {
}

func (c *UserController) Update(ctx *gin.Context) {
}

func (c *UserController) Delete(ctx *gin.Context) {
}

func (c *UserController) GetList(ctx *gin.Context) {
}
