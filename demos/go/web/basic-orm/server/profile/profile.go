package profile

const version = "v0.1.0"

type Profile struct {
	Mode    string `json:"mode"`
	Port    int    `json:"port"`
	Version string `json:"version"`
	Secret  string `json:"secret"`
}

func GetProfile() (*Profile, error) {
	profile := Profile{}

	if profile.Mode != "demo" && profile.Mode != "dev" && profile.Mode != "prod" {
		profile.Mode = "demo"
	}

	profile.Version = version

	return &profile, nil
}
