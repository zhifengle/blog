package basic

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
)

func md5V(str string) string {
	h := md5.New()
	h.Write([]byte(str))
	return hex.EncodeToString(h.Sum(nil))
}

func init() {
	author := "文和"
	for i := 0; i < 1e8; i++  {
		author = md5V(author)
	}
	fmt.Println(author)
}
