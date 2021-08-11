package basic

import "fmt"

func modifySlice(s []int) {
	s[0] = 1
}

func sliceMain() {
	fmt.Println("=== slice block ===")
	s1 := []int{1, 1, 1}
	s2 := make([]int, 3)

	fmt.Printf("%v\n", s1)
	fmt.Printf("%T\n", s2)
	modifySlice(s1)
	modifySlice(s2)
	//fmt.Println("%#v", s1)
	//fmt.Println("%#v", s2)
}
