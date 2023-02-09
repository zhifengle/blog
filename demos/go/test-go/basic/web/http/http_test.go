package http

import "testing"

func TestHttp(t *testing.T) {
	// get
	url := "http://www.baidu.com"
	body, err := Get(url)
	if err != nil {
		t.Error(err)
	}
	t.Log(body)

	// post
	url = "http://www.baidu.com"
	body, err = Post(url, []byte("name=cjb"))
	if err != nil {
		t.Error(err)
	}
	t.Log(body)
}
