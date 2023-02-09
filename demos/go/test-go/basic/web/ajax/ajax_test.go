package ajax

import (
	"net/url"
	"testing"
)

func TestReadNodeSiteConfig(t *testing.T) {
	config := readNodeSiteConfig()
	site := "bs.acgrip.com"
	siteConfig, ok := config[site]
	if !ok {
		return
	}
	t.Log(siteConfig.HttpsAgent)
}

func TestGet(t *testing.T) {
	url := "https://httpbin.org/ip"
	res, err := Get(url, nil)
	if err != nil {
		t.Error()
	}
	t.Log(res)
}

func TestPostForm(t *testing.T) {
	targetUrl := "https://httpbin.org/post"
	values := url.Values{}
	values.Add("custname", "testpost")
	res, err := PostForm(targetUrl, values, nil)
	if err != nil {
		t.Error()
	}
	t.Log(res)
}

func TestPostJson(t *testing.T) {
	targetUrl := "https://httpbin.org/post"
	values := []byte(`{"custname":"testpost"}`)
	res, err := PostJson(targetUrl, values, nil)
	if err != nil {
		t.Error()
	}
	t.Log(res)
}
