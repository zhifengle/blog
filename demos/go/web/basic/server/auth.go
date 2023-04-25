package server

import (
	"net/http"
	"web/basic/api"
	"web/basic/api/resp"
	"web/common"
	"web/common/r"

	"github.com/gin-gonic/gin"
	"golang.org/x/crypto/bcrypt"
)

func (s *Server) registerAuthRoutes(g *gin.RouterGroup, secret string) {
	g.POST("/auth/signin", func(c *gin.Context) {
		signin := &api.SignIn{}
		if err := c.ShouldBindJSON(signin); err != nil {
			c.JSON(http.StatusBadRequest, r.ResCode(r.ERROR_REQUEST_PARAM))
			return
		}
		userFind := &api.UserFind{
			Username: &signin.Username,
		}
		user, _ := s.Store.FindUser(c, userFind)
		if user == nil {
			c.JSON(http.StatusUnauthorized, r.ResCode(r.ERROR_USER_PASS_WRONG))
			return
		} else if user.RowStatus == api.Archived {
			c.JSON(http.StatusForbidden, r.ResCode(r.ERROR_USER_DISABLED))
			return
		}
		// Compare the stored hashed password, with the hashed version of the password that was received.
		if err := bcrypt.CompareHashAndPassword([]byte(user.PasswordHash), []byte(signin.Password)); err != nil {
			// If the two passwords don't match, return a 401 status.
			c.JSON(http.StatusUnauthorized, r.ResCode(r.ERROR_USER_PASS_WRONG))
			return
		}
		tokenInfo, err := GenerateTokens(c, user, secret)
		if err != nil {
			c.JSON(http.StatusInternalServerError, r.ResCode(r.FAIL))
			return
		}

		c.JSON(http.StatusOK, r.ResData(r.OK, &resp.LoginVO{
			AccessToken:  tokenInfo.AccessToken,
			RefreshToken: tokenInfo.RefreshToken,
			User:         *user,
		}))
	})

	g.POST("/auth/register", func(c *gin.Context) {
		signup := &api.SignUp{}
		if err := c.ShouldBindJSON(signup); err != nil {
			c.JSON(http.StatusBadRequest, r.ResCode(r.ERROR_REQUEST_PARAM))
			return
		}
		userCreate := &api.UserCreate{
			Username: signup.Username,
			// The new signup user should be normal user by default.
			Role:     api.NormalUser,
			Nickname: signup.Username,
			Password: signup.Password,
			OpenID:   common.GenUUID(),
		}
		hostUserType := api.Host
		existedHostUsers, _ := s.Store.FindUserList(c, &api.UserFind{
			Role: &hostUserType,
		})
		if len(existedHostUsers) == 0 {
			userCreate.Role = api.Host
		} else {
			// allowSignUpSetting, err := s.Store.FindSystemSetting(ctx, &api.SystemSettingFind{
			// 	Name: api.SystemSettingAllowSignUpName,
			// })
		}
		if err := userCreate.Validate(); err != nil {
			c.JSON(http.StatusBadRequest, r.ResCode(r.ERROR_REQUEST_PARAM))
			return
		}

		existedUser, _ := s.Store.FindUser(c, &api.UserFind{
			Username: &signup.Username,
		})
		if existedUser != nil {
			c.JSON(http.StatusBadRequest, r.ResCode(r.ERROR_USER_NAME_USED))
			return
		}
		passwordHash, err := bcrypt.GenerateFromPassword([]byte(signup.Password), bcrypt.DefaultCost)
		if err != nil {
			c.JSON(http.StatusInternalServerError, r.ResCode(r.FAIL))
			return
		}
		userCreate.PasswordHash = string(passwordHash)
		user, err := s.Store.CreateUser(c, userCreate)
		if err != nil {
			c.JSON(http.StatusInternalServerError, r.ResCode(r.FAIL))
			return
		}

		tokenInfo, err := GenerateTokens(c, user, secret)
		if err != nil {
			c.JSON(http.StatusInternalServerError, r.ResCode(r.FAIL))
			return
		}

		c.JSON(http.StatusOK, r.ResData(r.OK, &resp.LoginVO{
			AccessToken:  tokenInfo.AccessToken,
			RefreshToken: tokenInfo.RefreshToken,
			User:         *user,
		}))
	})
}
