package request

import (
	"compress/flate"
	"compress/gzip"
	"crypto/tls"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	urlPkg "net/url"
	"time"
)

// https://github.com/PuerkitoBio/goquery
// 用来解析 html ?

// http client
// https://github.com/go-resty/resty    4k star
// https://github.com/parnurzeal/gorequest   2k star

// 原名 annie
// https://github.com/iawia002/lux/blob/master/request/request.go
// 使用的内部变量来设置 Request 的一些参数

// 新增一个全局变量
var proxyUrl string

func SetProxy(url string) {
	proxyUrl = url
}

var defaultHeaders = map[string]string{
	"Accept":          "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
	"Accept-Charset":  "UTF-8,*;q=0.5",
	"Accept-Encoding": "gzip,deflate,sdch",
	"Accept-Language": "en-US,en;q=0.8",
	"User-Agent":      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.81 Safari/537.36",
}

// post url form; application/x-www-form-urlencoded
// netURL.QueryEscape(str)   // 转义字符
// payload := strings.NewReader(postData)
// res, err := request.Request(http.MethodPost, APIURL, payload, headers)

// post json; application/json
//values := map[string]string{"username": username, "password": password}
//jsonValue, _ := json.Marshal(values)
// bytes.NewBuffer(jsonValue)

func Request(method string, url string, body io.Reader, headers map[string]string) (*http.Response, error) {
	// body 传 json 时
	// b := bytes.NewReaders(s)

	//r := strings.NewReader("my request")
	//resp, err := http.Post("http://foo.bar", "application/x-www-form-urlencoded", r)

	// 重置
	defer SetProxy("")
	p := http.ProxyFromEnvironment
	if proxyUrl != "" {
		proxy, _ := urlPkg.Parse(proxyUrl)
		p = http.ProxyURL(proxy)
	}
	transport := &http.Transport{
		Proxy:               p,
		DisableCompression:  true,
		TLSHandshakeTimeout: 10 * time.Second,
		TLSClientConfig:     &tls.Config{InsecureSkipVerify: true},
	}
	client := &http.Client{
		Transport: transport,
		Timeout:   15 * time.Minute,
	}

	req, err := http.NewRequest(method, url, body)
	if err != nil {
		return nil, err
	}
	for k, v := range defaultHeaders {
		req.Header.Set(k, v)
	}
	for k, v := range headers {
		req.Header.Set(k, v)
	}
	// add cookie
	//req.Header.Set("Cookie", cookie)
	//req.Header.Set("Referer", refer)

	// https://stackoverflow.com/questions/12130582/setting-cookies-with-net-http
	// cookie will get expired after 1 year
	//expires := time.Now().AddDate(1, 0, 0)
	//ck := http.Cookie{
	//	Name: "JSESSION_ID",
	//	Domain: "foo.com",
	//	Path: "/",
	//	Expires: expires,
	//}
	//ck.Value = "value of this awesome cookie"
	//req.AddCookie(&ck)

	var (
		res          *http.Response
		requestError error
		// 改成配置?
		retryTimes = 2
	)
	for i := 0; ; i++ {
		res, requestError = client.Do(req)
		if requestError == nil && res.StatusCode < 400 {
			break
		} else if i+1 >= retryTimes {
			var err error
			if requestError != nil {
				err = fmt.Errorf("request error: %v", requestError)
			} else {
				err = fmt.Errorf("%s request error: HTTP %d", url, res.StatusCode)
			}
			return nil, err
		}
		time.Sleep(1 * time.Second)
	}

	return res, nil
}

func Get(url string, headers map[string]string) (string, error) {
	// 基本的写法
	//res, err := Request("GET", url, nil, headers)
	//defer res.Body.Close()
	//body, _ := ioutil.ReadAll(res.Body)
	//return string(body), err

	body, err := GetByte(url, headers)
	return string(body), err
}

// 获取相应字节?

// GetByte get request
func GetByte(url string, headers map[string]string) ([]byte, error) {
	if headers == nil {
		headers = map[string]string{}
	}
	res, err := Request(http.MethodGet, url, nil, headers)
	if err != nil {
		return nil, err
	}
	defer res.Body.Close() // nolint

	var reader io.ReadCloser
	switch res.Header.Get("Content-Encoding") {
	case "gzip":
		reader, _ = gzip.NewReader(res.Body)
	case "deflate":
		reader = flate.NewReader(res.Body)
	default:
		reader = res.Body
	}
	defer reader.Close() // nolint

	body, err := ioutil.ReadAll(reader)
	if err != nil && err != io.EOF {
		return nil, err
	}
	return body, nil
}
