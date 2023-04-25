package server

import (
	"errors"
	"fmt"
	"net/http"
	"strings"
	"web/basic/api"
	"web/common"
	"web/common/r"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
)

type TokenInfo struct {
	AccessToken  string `json:"access_token"`
	RefreshToken string `json:"refresh_token"`
}

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

func GenerateTokens(c *gin.Context, user *api.User, secret string) (*TokenInfo, error) {
	accessToken, err := common.GenerateAccessToken(user.ID, user.Username, secret)
	if err != nil {
		return nil, fmt.Errorf("failed to generate access token %w", err)
	}

	refreshToken, err := common.GenerateRefreshToken(user.ID, user.Username, secret)
	if err != nil {
		return nil, fmt.Errorf("failed to generate refresh token %w", err)
	}

	return &TokenInfo{
		AccessToken:  accessToken,
		RefreshToken: refreshToken,
	}, nil
}

func SetTokenCookies(c *gin.Context, tokenInfo *TokenInfo) {
	cookieMaxAge := int(common.CookieExpDuration.Seconds())
	c.SetCookie(common.AccessTokenCookieName, tokenInfo.AccessToken, cookieMaxAge, "", c.Request.Host, true, true)
	c.SetCookie(common.RefreshTokenCookieName, tokenInfo.RefreshToken, cookieMaxAge, "", c.Request.Host, true, true)
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
