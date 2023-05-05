package api

import "fmt"

type SignIn struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

type SignUp struct {
	SignIn
}

type UserCreate struct {
	// Domain specific fields
	Username     string `json:"username"`
	Email        string `json:"email"`
	Nickname     string `json:"nickname"`
	Password     string `json:"password"`
	PasswordHash string
}

// @TODO validate
func (create UserCreate) Validate() error {
	if len(create.Username) < 3 {
		return fmt.Errorf("username is too short, minimum length is 3")
	}
	return nil
}

type UserPatch struct {
	ID int `json:"-"`

	Username *string `json:"username"`
	Email    *string `json:"email"`
	Nickname *string `json:"nickname"`
	Password *string `json:"password"`
}

// @TODO validate
func (patch UserPatch) Validate() error {
	if patch.Username != nil && len(*patch.Username) < 3 {
		return fmt.Errorf("username is too short, minimum length is 3")
	}
	return nil
}
