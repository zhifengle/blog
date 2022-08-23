package main

import (
	"bytes"
	"fmt"
	"reflect"
	"unsafe"
)

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
	fmt.Printf("%#v\n", s1)
	fmt.Printf("%#v\n", s2)
}
func main() {
	deepEqual()
}

// https://coolshell.cn/articles/21128.html
type data struct {
	array unsafe.Pointer //指向存放数据的数组指针
	len   int            //长度有多大
	cap   int            //容量有多大
}

func sliceDemo() {
	path := []byte("AAAA/BBBBBBBBB")
	sepIndex := bytes.IndexByte(path, '/')

	// 全量切片
	//dir1 := path[:sepIndex:sepIndex]
	dir1 := path[:sepIndex]
	dir2 := path[sepIndex+1:]
	fmt.Println("dir1 =>", string(dir1))
	fmt.Println("dir2 =>", string(dir2))

	dir1 = append(dir1, "suffix"...)
	// dir1 影响到了 dir2. 改成全量切片
	fmt.Println("dir1 =>", string(dir1))
	fmt.Println("dir2 =>", string(dir2))
}

func deepEqual() {
	v1 := data{}
	v2 := data{}
	fmt.Println("v1 == v2:", reflect.DeepEqual(v1, v2))
	//prints: v1 == v2: true
	m1 := map[string]string{"one": "a", "two": "b"}
	m2 := map[string]string{"two": "b", "one": "a"}
	fmt.Println("m1 == m2:", reflect.DeepEqual(m1, m2))
	//prints: m1 == m2: true
	s1 := []int{1, 2, 3}
	s2 := []int{1, 2, 3}
	fmt.Println("s1 == s2:", reflect.DeepEqual(s1, s2))
	//prints: s1 == s2: true
}

// -------------------------------------------------

type City struct {
	Name string
}
type Country struct {
	Name string
}
type Printable interface {
	PrintStr()
}

func (c City) PrintStr() {
	fmt.Println(c.Name)
}
func (c Country) PrintStr() {
	fmt.Println(c.Name)
}

func printTest() {
	c1 := City{"Beijing"}
	c2 := Country{Name: "China"}
	c1.PrintStr()
	c2.PrintStr()

	d1 := Country{"USA"}
	d2 := City{"Los Angeles"}

	printStr(d1)
	printStr(d2)
}

type Stringable interface {
	ToString() string
}

func (c Country) ToString() string {
	return "Country = " + c.Name
}
func (c City) ToString() string {
	return "Country = " + c.Name
}

func printStr(p Stringable) {
	fmt.Println(p.ToString())
}
