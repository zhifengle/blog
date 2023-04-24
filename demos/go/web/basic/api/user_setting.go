package api

type UserSetting struct {
	UserID int
	Key    string `json:"key"`
	Value  string `json:"value"`
}
