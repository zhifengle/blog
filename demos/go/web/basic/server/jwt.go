package server

import (
	"errors"
	"net/http"
	"strings"
	"web/common"
	"web/common/r"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
)

func extractTokenFromHeader(header http.Header) string {
	authHeader := header.Get("Authorization")
	if authHeader == "" {
		return ""
	}

	authHeaderParts := strings.Fields(authHeader)
	if len(authHeaderParts) != 2 || strings.ToLower(authHeaderParts[0]) != "bearer" {
		return ""
	}

	return authHeaderParts[1]
}

func findAccessToken(c *gin.Context) string {
	accessToken := ""
	cookie, _ := c.Cookie(common.AccessTokenCookieName)
	if cookie != "" {
		accessToken = cookie
	}
	if accessToken == "" {
		accessToken = extractTokenFromHeader(c.Request.Header)
	}

	return accessToken
}

// JWTMiddleware validates the access token.
// If the access token is about to expire or has expired and the request has a valid refresh token, it
// will try to generate new access token and refresh token.
func JWTMiddleware(server *Server, secret string) gin.HandlerFunc {
	return func(c *gin.Context) {

		if server.IsPublic(c) {
			c.Next()
			return
		}

		token := findAccessToken(c)
		if token == "" {
			c.AbortWithStatusJSON(http.StatusUnauthorized, r.ResCode(r.ERROR_TOKEN_NOT_EXIST))
			return
		}
		_, err := common.ParseToken(token, secret)
		if err != nil {
			if errors.Is(err, jwt.ErrTokenExpired) {
				c.AbortWithStatusJSON(200, r.ResCode(r.ERROR_TOKEN_RUNTIME))
				return
			}
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error})
			c.Abort()
			return
		}
		c.Next()
	}
}
