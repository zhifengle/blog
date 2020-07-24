package basic

import (
	"encoding/hex"
	"golang.org/x/crypto/md4"
)

func md5V(str string) string {
	h := md4.New()
	h.Write([]byte(str))
	return hex.EncodeToString(h.Sum(nil))
}

func ans(str string) string {
	// 07/24
	//author := "李兆廷"
	//author := "独角龙"
	res := str
	for i := 0; i < 1e8; i++ {
		res = md5V(res)
	}
	return res
}
