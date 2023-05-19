package file

import (
	"bufio"
	"os"
	"path/filepath"
)

func ReadFile(filename string) (string, error) {
	s, err := os.ReadFile(filename)
	return string(s), err
}

func ReadLines(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

func IsFileExist(filename string) bool {
	info, err := os.Stat(filename)
	if os.IsNotExist(err) {
		return false
	}
	if err != nil {
		return false
	}
	return !info.IsDir()
}

func GetCurrentPath() (string, error) {
	return os.Getwd()
}

func FindTextFiles(dir string) ([]string, error) {
	var files []string
	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && filepath.Ext(path) == ".txt" {
			files = append(files, path)
		}
		return nil
	})
	return files, err
}

func Basename(path string) string {
	return filepath.Base(path)
}

func Dirname(path string) string {
	return filepath.Dir(filepath.Clean(path))
}

func EnsureDir(path string) error {
	return os.MkdirAll(path, 0755)
}
