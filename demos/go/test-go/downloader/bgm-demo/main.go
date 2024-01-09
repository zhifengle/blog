package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"regexp"
	"time"
)

func ReadFileLines(textFile string) ([]string, error) {
	file, err := os.Open(textFile)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()
		lines = append(lines, text)
	}
	return lines, scanner.Err()
}

func LoadCookies() string {
	// check if .cookies exists
	if _, err := os.Stat(".cookies"); err != nil {
		return ""
	}
	cookies, err := os.ReadFile(".cookies")
	if err != nil {
		return ""
	}
	return string(cookies)
}

func main() {
	headers := make(map[string]string)
	cookie := LoadCookies()
	if cookie == "" {
		log.Fatal("no file .cookies")
	}
	headers["cookie"] = cookie
	uaList, err := ReadFileLines("ua.txt")
	if err != nil {
		log.Fatal(err)
	}
	i := 0
	pattern := `<small class="grey rr">online: (\d+)</small>`
	re := regexp.MustCompile(pattern)
	for {
		headers["user-agent"] = uaList[i]
		fmt.Println("request bgm.tv with UA: ", uaList[i])
		content := request("https://bgm.tv/", headers)
		match := re.FindStringSubmatch(string(content))
		if len(match) > 1 {
			fmt.Println("online number:", match[1])
		} else {
			fmt.Println("no online number")
			break
		}
		time.Sleep(time.Minute * 10)
		i += 1
		if i >= len(uaList) {
			i = 0
		}
	}
}

func request(url string, headers map[string]string) []byte {
	client := &http.Client{}
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatal(err)
	}
	req.Header.Set("authority", "bgm.tv")
	req.Header.Set("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
	// req.Header.Set("accept-language", "en")
	req.Header.Set("cache-control", "no-cache")
	// req.Header.Set("cookie", "mycookie")
	req.Header.Set("pragma", "no-cache")
	// req.Header.Set("sec-ch-ua", `"Not)A;Brand";v="24", "Chromium";v="116"`)
	// req.Header.Set("sec-ch-ua-mobile", "?0")
	// req.Header.Set("sec-ch-ua-platform", `"Windows"`)
	// req.Header.Set("sec-fetch-dest", "document")
	// req.Header.Set("sec-fetch-mode", "navigate")
	// req.Header.Set("sec-fetch-site", "none")
	// req.Header.Set("sec-fetch-user", "?1")
	// req.Header.Set("upgrade-insecure-requests", "1")
	for k, v := range headers {
		req.Header.Set(k, v)
	}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()
	bodyText, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal(err)
	}
	return bodyText
}
