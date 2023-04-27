package profile

import "fmt"

const version = "v0.1.0"

type Profile struct {
	Mode    string `json:"mode"`
	Port    int    `json:"port"`
	DSN     string `json:"dsn"`
	Version string `json:"version"`
	Data    string `json:"-"`
	Secret  string `json:"secret"`
}

func GetProfile(name string) (*Profile, error) {
	profile := Profile{}

	if profile.Mode != "demo" && profile.Mode != "dev" && profile.Mode != "prod" {
		profile.Mode = "demo"
	}

	if profile.Mode == "prod" && profile.Data == "" {
		profile.Data = "/var/opt/" + name
	}

	dataDir := "."

	profile.Data = dataDir
	profile.DSN = fmt.Sprintf("%s/%s_%s.db", dataDir, name, profile.Mode)
	profile.Version = version

	return &profile, nil
}
