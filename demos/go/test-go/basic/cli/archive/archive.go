package main

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
)

var (
	excludeDirs = []string{"done"}
)

func main() {
	password := flag.String("p", "", "password")
	single := flag.Bool("s", false, "single file")
	flag.Parse()
	args := flag.Args()
	if len(args) == 0 {
		flag.Usage()
		return
	}
	if password == nil || *password == "" {
		fmt.Println("password is required")
		return
	}
	if *single {
		run7z(args[0], *password)
		return
	}
	files, err := os.ReadDir(args[0])
	if err != nil {
		fmt.Println(err)
		return
	}
	for _, file := range files {
		if file.IsDir() && !contains(excludeDirs, file.Name()) {
			path := args[0] + "/" + file.Name()
			run7z(path, *password)
		}
	}
}

func run7z(path string, password string) {
	cmd := exec.Command("7z", "a", path+".7z", "-mhe", "-p"+password, path)
	fmt.Println("compressing: ", path)
	err := cmd.Run()
	if err != nil {
		fmt.Println(err)
	}
}

func contains(list []string, item string) bool {
	for _, v := range list {
		if v == item {
			return true
		}
	}
	return false
}
