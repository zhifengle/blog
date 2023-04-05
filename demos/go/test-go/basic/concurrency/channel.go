package concurrency

import (
	"fmt"
	"time"
)

func printNums() {
	done := make(chan int, 10)
	for i := 0; i < cap(done); i++ {
		go func(i int) {
			fmt.Println("Hello from goroutine", i)
			done <- i
		}(i)
	}
	for i := 0; i < cap(done); i++ {
		<-done
	}
}

// 1000 tasks, 10 goroutines
func processTasks() {
	var ch = make(chan int, 10)
	for i := 0; i < 1000; i++ {
		go func(task int) {
			fmt.Println("Hello from goroutine: ", task)
			ch <- task
		}(i)
	}
	for i := 0; i < 1000; i++ {
		res := <-ch
		fmt.Println("Result: ", res)
	}
}

// 推荐使用 done chan
func infiniteLoop() {
	for {
		select {
		default:
			fmt.Println("doing work")
		}
	}
}

func doWork(done <-chan bool) {
	for {
		select {
		case <-done:
			return
		default:
			fmt.Println("doing")
		}
	}
}
func donePattern() {
	done := make(chan bool)
	go doWork(done)

	time.Sleep(time.Second * 1)
	// done <- true
	close(done)
}
