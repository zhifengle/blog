package server

import (
	"web/basic-orm/model/api"
	"web/basic-orm/services"
	"web/common/web"

	"github.com/gin-gonic/gin"
)

func userListHandler(c *gin.Context) {
	paging := &api.Paging{}
	if err := c.ShouldBindQuery(paging); err != nil {
		c.JSON(200, web.JsonErrorCode(web.ERROR_INVALID_PARAM))
		return
	}
	result, err := services.UserService.GetList(paging)
	if err != nil {
		c.JSON(500, web.JsonErrorCode(web.FAIL))
	}
	c.JSON(200, web.JsonData(result))
}

func (s *Server) registerUserRoutes(g *gin.RouterGroup) {
	g.GET("/user/list", userListHandler)
}
