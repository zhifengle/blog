package server

import (
	"web/basic/api"

	"github.com/gin-gonic/gin"
)

func (s *Server) registerAuthRoutes(g *gin.RouterGroup, secret string) {
	g.POST("/auth/signin", func(ctx *gin.Context) {
		signin := &api.SignIn{}
		if err := ctx.ShouldBindJSON(signin); err != nil {
			ctx.JSON(400, gin.H{
				"error": err.Error(),
			})
			return
		}

		if signin.Username == "admin" && signin.Password == "admin" {
			ctx.JSON(200, gin.H{
				"code": 0,
				"msg":  "success",
			})
		} else {
			ctx.JSON(200, gin.H{
				"code": 1,
				"msg":  "fail",
			})
		}
	})
}
