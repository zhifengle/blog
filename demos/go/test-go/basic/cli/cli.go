package main

import (
	"flag"
	"fmt"
)

func main() {
	var version = "1"
	var help = `
Some help file` + version + `
参数：
    -n 200
        test count default 200
    -h
        print help
`
	// 在 program args 里面配置 -n 101
	flag.IntVar(&Routines, "n", 200, "测速线程数量")

	flag.Usage = func() { fmt.Print(help) }
	flag.Parse()
	fmt.Println("Routines: ", Routines)
}
