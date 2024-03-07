package main

import (
	"bytes"
	"log"
	"os"
	"strings"
)

func main() {
	args := os.Args[1:]
	if len(args) < 1 {
		log.Fatal("Please provide an input file path.")
	}

	inPath := args[0]
	outPath := strings.TrimSuffix(inPath, ".exe") + "-no.exe"

	exeData, err := os.ReadFile(inPath)
	if err != nil {
		log.Fatalf("Failed to read input file: %v", err)
	}

	exeHeader := exeData[:4] // 4D 5A 90 00
	startLocation := bytes.Index(exeData[1:], exeHeader)
	if startLocation == -1 {
		log.Fatal("header not found.")
	}

	err = os.WriteFile(outPath, exeData[startLocation+1:], 0644)
	if err != nil {
		log.Fatalf("Failed to write cleaned file: %v", err)
	}
}
