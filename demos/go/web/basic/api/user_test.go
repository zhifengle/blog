package api

import "testing"

func TestRole(t *testing.T) {
	if Admin.String() != "ADMIN" {
		t.Error("Role() failed.")
	}
}
