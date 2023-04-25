package resp

import "web/basic/api"

type LoginVO struct {
	AccessToken  string `json:"accessToken"`
	RefreshToken string `json:"refreshToken"`
	api.User
}
