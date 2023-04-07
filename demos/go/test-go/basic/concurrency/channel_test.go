package concurrency

import (
	"fmt"
	"testing"
	"time"
)

func TestPrintNums(t *testing.T) {
	go printNums()
}

// test processTasks
func TestProcessTasks(t *testing.T) {
	go processTasks()
}

func TestDoPattern(t *testing.T) {
	donePattern()
}

func TestLimitRun(t *testing.T) {
	now := time.Now()
	limitRun(3, []string{"1", "2", "3", "4", "5", "6", "7", "8"}, fakeWorkFn)
	fmt.Println("finished: ", time.Since(now))
}
