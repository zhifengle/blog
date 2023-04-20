package common

import (
	"io"
	"net/http"
	"os"

	"github.com/cheggaaa/pb/v3"
)

var client = &http.Client{}

// usage: common.Download("http://78.142.195.195/100MB.test", "100MB.test")
func Download(url string, target string) error {
	res, err := client.Get(url)
	if err != nil {
		return err
	}
	defer res.Body.Close()

	output, err := os.Create(target)
	if err != nil {
		return err
	}
	defer output.Close()

	bar := pb.Full.Start64(res.ContentLength)
	// bar.Set(pb.Bytes, true)
	barReader := bar.NewProxyReader(res.Body)
	io.Copy(output, barReader)

	return nil
}
