package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type Config struct {
	Filename string
	Query    string
}

func parseConfig(args []string) Config {
	if len(args) < 2 {
		log.Fatal("not enough arguments")
	}
	config := Config{
		Query:    args[0],
		Filename: args[1],
	}
	return config
}

//func search(query, contents string) []string {
//}

func main() {
	// https://gobyexample.com/
	// flag 包是  -name=xxx
	args := os.Args[1:]
	config := parseConfig(args)
	// 直接读取整个文件
	//dat, err := ioutil.ReadFile("poem.txt")
	//check(err)
	//string(dat)

	// 一行行读取
	// https://stackoverflow.com/questions/8757389/reading-a-file-line-by-line-in-go
	//file, err := os.Open("poem.txt")
	file, err := os.Open(config.Filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var res []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		txt := scanner.Text()
		//fmt.Println(scanner.Text())
		if strings.Contains(txt, config.Query) {
			res = append(res, txt)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Println(strings.Join(res, "\n"))
}
