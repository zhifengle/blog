package iodemo

import (
	"bytes"
	"crypto/md5"
	"crypto/sha256"
	"fmt"
	"hash"
	"io"
	"os"
	"time"
)

// 边读边计算 md5. 演示用，不一定正确
func TeeGetFileMD5(path string) (string, error) {
	file, err := os.Open(path)
	if err != nil {
		return "", err
	}
	h := md5.New()
	tr := io.TeeReader(file, h)
	_, err = io.ReadAll(tr) // 优化
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("%x", h.Sum(nil)), nil
}

func TestMultiReader() {
	r1 := bytes.NewReader([]byte("ABC"))
	r2 := bytes.NewReader([]byte("DEF"))
	reader := io.MultiReader(r1, r2)
	var buf = make([]byte, 1)
	for {
		n, err := reader.Read(buf)
		if err != nil {
			if err == io.EOF {
				return
			}
			fmt.Println(err)
			return
		}
		fmt.Println(string(buf[:n])) // ABCDEF
	}
}

func TestMultiWriter() {
	var buf []byte
	w1 := bytes.NewBuffer(buf)
	w2 := bytes.NewBuffer(buf)
	writer := io.MultiWriter(w1, w2)
	_, err := writer.Write([]byte("123"))
	if err != nil {
		fmt.Println(err)
		return
	}

	w1Res, err := io.ReadAll(w1)
	fmt.Println(string(w1Res), err) // 123 <nil>
	w2Res, err := io.ReadAll(w2)
	fmt.Println(string(w2Res), err) // 123 <nil>
}

// ----------------------------------------
func CopyFileWithHash() {
	f, dstF, hashW, err := getTestRW()
	if err != nil {
		fmt.Println(err)
		return
	}

	now := time.Now()
	defer func() {
		fmt.Println("耗时：", time.Since(now))
	}()

	multiW := io.MultiWriter(dstF, hashW)

	teeR := io.TeeReader(f, multiW)

	buf := make([]byte, 512)
	for {
		_, err := teeR.Read(buf)
		if err == io.EOF {
			break
		}
		// utils.CheckErr(err)
	}

	fmt.Printf("文件sha256：%x\n", hashW.Sum(nil))
}

func getTestRW() (f, dstF *os.File, shaW hash.Hash, err error) {
	f, err = os.Open(`xxx.txt`)
	if err != nil {
		return
	}
	fInfo, err := f.Stat()
	if err != nil {
		return
	}
	fmt.Println("文件大小：", fInfo.Size())

	dstF, err = os.Create(`demo.tar`)
	if err != nil {
		return
	}

	shaW = sha256.New()
	return
}
