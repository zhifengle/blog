package main

import (
	"fmt"
	"io"
	"mime"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"test_go/downloader/request"
)

// 10M 分片
// 3495254 * 2 + 3495252

type FilePartMeta struct {
	Index int
	Start int64
	End   int64
	Cur   int64
}

func getPartListByNum(total int64, num int64) []FilePartMeta {
	partSize := (total + num - 1) / num
	partList := make([]FilePartMeta, num)
	for i := int64(0); i < num; i++ {
		start := i * partSize
		end := start + partSize - 1
		if end > total-1 {
			end = total - 1
		}
		partList[i] = FilePartMeta{
			Index: int(i),
			Start: start,
			End:   end,
			Cur:   start,
		}
	}
	return partList
}

func getFileName(resp *http.Response) string {
	contentDisposition := resp.Header.Get("Content-Disposition")
	if contentDisposition != "" {
		_, params, err := mime.ParseMediaType(contentDisposition)

		if err == nil {
			return params["filename"]
		}
	}
	filename := filepath.Base(resp.Request.URL.Path)
	return filename
}

func downloadPart(url string, part *FilePartMeta, file *os.File) error {
	header := map[string]string{
		"Range": fmt.Sprintf("bytes=%v-%v", part.Cur, part.End),
	}
	res, err := request.Request("GET", url, nil, header)
	if err != nil {
		return err
	}
	defer res.Body.Close()
	_, err = io.Copy(file, res.Body)
	return err
}

func mergeFile(fileName string, parts []*os.File) error {
	outputFile, err := os.Create(fileName)
	if err != nil {
		return fmt.Errorf("creating output file: %s", err)
	}
	defer outputFile.Close()

	for _, part := range parts {
		_, err := part.Seek(0, 0)
		if err != nil {
			return fmt.Errorf("seeking to the beginning of part: %s", err)
		}

		_, err = io.Copy(outputFile, part)
		if err != nil {
			return fmt.Errorf("writing part to output file: %s", err)
		}

		part.Close()
	}

	return nil
}

func Download(url string, fileName string, totalSize int64, num int) error {
	parts := make([]*os.File, num)
	metas := getPartListByNum(totalSize, int64(num))

	// Create a wait group to wait for all parts to finish downloading
	var wg sync.WaitGroup
	wg.Add(num)
	for i := 0; i < num; i++ {
		partFile, err := os.Create(fmt.Sprintf("%v-%v.tmp", fileName, i))
		if err != nil {
			return fmt.Errorf("creating part file: %s", err)
		}
		parts[i] = partFile
		go func(i int) {
			defer wg.Done()
			err := downloadPart(url, &metas[i], partFile)
			if err != nil {
				fmt.Println(err)
			}
		}(i)
	}
	wg.Wait()
	return mergeFile(fileName, parts)
}

func main() {
	url := "https://proof.ovh.net/files/10Mb.dat"
	res, err := request.Request("HEAD", url, nil, nil)
	if err != nil {
		fmt.Println(err)
	}
	fileSize := res.ContentLength
	supportRange := res.Header.Get("Accept-Ranges") == "bytes"
	if supportRange {
		fmt.Printf("支持断点续传，文件大小为: %v\n", fileSize)
	} else {
		fmt.Printf("不支持断点续传，文件大小为: %v\n", fileSize)
	}
	fileName := getFileName(res)
	err = Download(url, fileName, fileSize, 3)
	if err != nil {
		fmt.Println(err)
	}
}
