package request

import (
	"fmt"
	"testing"
)

func TestGet(t *testing.T) {
	var err error
	type args struct {
		url     string
		headers map[string]string
	}
	tests := []struct {
		name string
		args args
	}{
		//{
		//	name: "baidu",
		//	args: args{
		//		url:     "https://www.baidu.com/",
		//		headers: nil,
		//	},
		//},
		//{
		//	name: "bing",
		//	args: args{
		//		url: "https://cn.bing.com/",
		//		headers: map[string]string{
		//			"Referer": "https://cn.bing.com",
		//		},
		//	},
		//},
		{
			name: "52pojie",
			args: args{
				url: "https://www.52pojie.cn/",
				headers: map[string]string{
					"Referer": "https://www.52pojie.cn",
				},
			},
		},
	}
	for _, tt := range tests {
		//
		t.Run(tt.name, func(t *testing.T) {
			if tt.name == "52pojie" {
				// 添加 go tool -v 参数可以打印
				body, _ := GetByte(tt.args.url, tt.args.headers)
				s, _ := GbkToUtf8(body)
				fmt.Println(string(s))
			} else {
				_, err = GetByte(tt.args.url, tt.args.headers)
			}
			if err != nil {
				t.Error()
			}
		})
		// 打印错误
		//t.Fatalf("tests[%d] - something wrong. expected=%q, got=%q", 1, "AAAAA", "BBB")
	}
}
