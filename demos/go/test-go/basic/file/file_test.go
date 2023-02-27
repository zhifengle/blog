package file

import (
	"os"
	"testing"
)

func TestFile(t *testing.T) {
	// ReadLines
	lines, err := ReadLines("file.go")
	if err != nil {
		t.Error(err)
	}
	t.Log(lines)

	// IsFileExist
	t.Log(IsFileExist("file.go"))

	// GetCurrentPath
	path, err := GetCurrentPath()
	if err != nil {
		t.Error(err)
	}
	t.Log(path)
}

func TestBaseAndDirName(t *testing.T) {
	homeDir, _ := os.UserHomeDir()
	// config := homeDir + `\.config`
	config := homeDir + "/.config"
	// Basename
	t.Log(Basename(config))
	// Dirname
	t.Log(Dirname(config))
}
