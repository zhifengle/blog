package main

import (
	"crypto/aes"
	"crypto/cipher"
	"encoding/hex"
	"fmt"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

// https://gobyexample.com/reading-files
func main() {
	// s := []byte("Hello 世界")
	//key, err := os.ReadFile("key.bin")
	//value, err := os.ReadFile("encrypted_value.bin")
	keyStr := "xxx"
	valueStr := "bbb"
	key, err := hex.DecodeString(keyStr)
	check(err)
	value, err := hex.DecodeString(valueStr)
	check(err)
	block, _ := aes.NewCipher(key)
	blockMode, _ := cipher.NewGCM(block)
	origData, _ := blockMode.Open(nil, value[3:15], value[15:], nil)
	fmt.Println(string(origData))
}
