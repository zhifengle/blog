package common

import (
	"testing"
	"time"
)

func TestParseToken(t *testing.T) {
	// minus a hour
	expirationTime := time.Now().Add(-time.Hour)
	tokenString, _ := generateToken(1, "foo", AccessTokenAudienceName, expirationTime, "secret")
	t.Log(tokenString)
	claim, err := ParseToken(tokenString, "secret")
	if err != nil {
		t.Error(err)
	}
	t.Log(claim)
}
