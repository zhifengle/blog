package file

import "testing"

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
