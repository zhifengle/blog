package basic

import (
	"fmt"
	"reflect"
)

func init() {
	author := "draven"
	fmt.Println("TypeOf author:", reflect.TypeOf(author))
	fmt.Println("ValueOf author:", reflect.ValueOf(author))
	r := Rectangle{1, 2}
	fmt.Println(r.area())
}
