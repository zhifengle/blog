package concurrency

import (
	"fmt"
	"sync"
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
func printNums2() {
	c := make(chan int)

	go func() {
		for i := 0; i < 10; i++ {
			c <- i
		}
		close(c)
	}()

	for i := range c {
		fmt.Println(i)
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

func fakeWorkFn(url string) {
	time.Sleep(time.Second * 1)
	fmt.Println("req: {}", url)
}

func limitRun[T any](num int, arr []T, iterFn func(item T)) {
	var wg sync.WaitGroup
	control := make(chan bool, num)
	for _, item := range arr {
		wg.Add(1)
		go func(item T) {
			defer wg.Done()
			control <- false
			iterFn(item)
			<-control
		}(item)
	}
	wg.Wait()
}

// ref: https://gobyexample.com/channel-buffering

func worker(done chan bool) {
	fmt.Print("working...")
	time.Sleep(time.Second)
	fmt.Println("done")

	done <- true
}
func waitWorker() {
	done := make(chan bool, 1)
	go worker(done)

	// block until worker is done
	<-done
}
