package basic

import (
	"encoding/json"
	"fmt"
)

type student struct {
	//Name  string `json:"name"`

	//这种输出和上面的 string 一样，只是支持设置为数字
	Name  interface{} `json:"name"`
	Age   int         `json:"age"`
	Class *class      `json:"class"`
}
type class struct {
	Name  string `json:"name"`
	Grade int    `json:"grade"`
}

func OutputJson() {
	st := student{
		Name: "zhangsan",
		Age:  18,
	}
	cls := new(class)
	cls.Name = "1班"
	cls.Grade = 3

	st.Class = cls

	str, err := json.Marshal(st)
	if err != nil {
		fmt.Errorf("gen json err")
	}

	fmt.Println(string(str))

	// 测试
	data := "{\"name\":\"张三\",\"Age\":18,\"high\":true,\"sex\":\"男\",\"CLASS\":{\"naME\":\"2班\",\"GradE\":5}}"
	str2 := []byte(data)

	stu := student{}
	err = json.Unmarshal(str2, &stu)

	//解析失败会报错，如json字符串格式不对，缺"号，缺}等。
	if err != nil {
		fmt.Println(err)
	}

	//fmt.Println(stu)
	str, err = json.Marshal(stu)
	if err != nil {
		fmt.Errorf("gen json err")
	}

	fmt.Println(string(str))
}
