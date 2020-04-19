package basic

import "math"

type person struct {
	name string
	age  int
}

func Older(p1, p2 person) person {
	if p1.age > p2.age {
		return p1
	}
	return p2
}

type Rectangle struct {
	width, height float64
}
type Circle struct {
	radius float64
}

func (r Rectangle) area() float64 {
	return r.width * r.height
}

func (r Circle) area() float64 {
	return r.radius * r.radius * math.Pi
}
