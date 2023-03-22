package concurrency

import "testing"

func TestPrintNums(t *testing.T) {
	go printNums()
}

// test processTasks
func TestProcessTasks(t *testing.T) {
	go processTasks()
}
