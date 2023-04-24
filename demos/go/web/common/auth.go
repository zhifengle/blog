package common

import (
	"fmt"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

const (
	issuer = "myapp"
	keyID  = "v1"
	// AccessTokenAudienceName is the audience name of the access token.
	AccessTokenAudienceName = "user.access-token"
	// RefreshTokenAudienceName is the audience name of the refresh token.
	RefreshTokenAudienceName = "user.refresh-token"
	accessTokenDuration      = 24 * time.Hour
	refreshTokenDuration     = 7 * 24 * time.Hour
	// RefreshThresholdDuration is the threshold duration for refreshing token.
	RefreshThresholdDuration = 1 * time.Hour

	AccessTokenCookieName  = "access-token"
	RefreshTokenCookieName = "refresh-token"
	CookieExpDuration      = refreshTokenDuration - 1*time.Minute
)

type CustomClaims struct {
	Username string `json:"username"`
	UserId   int    `json:"user_id"`
	jwt.RegisteredClaims
}

func generateToken(userId int, username string, aud string, expirationTime time.Time, secret string) (string, error) {
	claims := &CustomClaims{
		Username: username,
		RegisteredClaims: jwt.RegisteredClaims{
			Audience:  jwt.ClaimStrings{aud},
			ExpiresAt: jwt.NewNumericDate(expirationTime),
			IssuedAt:  jwt.NewNumericDate(time.Now()),
			Issuer:    issuer,
			// Subject:   "some",
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	token.Header["kid"] = keyID

	tokenString, err := token.SignedString([]byte(secret))
	if err != nil {
		return "", err
	}

	return tokenString, nil
}

// GenerateAccessToken generates an access token for web.
func GenerateAccessToken(userId int, userName, secret string) (string, error) {
	expirationTime := time.Now().Add(accessTokenDuration)
	return generateToken(userId, userName, AccessTokenAudienceName, expirationTime, secret)
}

// GenerateRefreshToken generates a refresh token for web.
func GenerateRefreshToken(userId int, userName, secret string) (string, error) {
	expirationTime := time.Now().Add(refreshTokenDuration)
	return generateToken(userId, userName, RefreshTokenAudienceName, expirationTime, secret)
}

func ParseToken(tokenString string, secret string) (*CustomClaims, error) {
	claims := &CustomClaims{}
	token, err := jwt.ParseWithClaims(tokenString, claims, func(t *jwt.Token) (any, error) {
		if t.Method.Alg() != jwt.SigningMethodHS256.Name {
			return nil, fmt.Errorf("unexpected access token signing method=%v, expect %v", t.Header["alg"], jwt.SigningMethodHS256)
		}
		if kid, ok := t.Header["kid"].(string); ok {
			if kid == "v1" {
				return []byte(secret), nil
			}
		}
		return nil, fmt.Errorf("unexpected access token kid=%v", t.Header["kid"])
	})
	if token.Valid {
		return claims, nil
	}

	return nil, err
}
