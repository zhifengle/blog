package concurrency

import "testing"

func TestWaitGroup(t *testing.T) {
	wgDemo()
	wgHello()
}

func TestWgPrintNums(t *testing.T) {
	wgPrintNums()
}

func TestWgTasks(t *testing.T) {
	wgTasks()
}
