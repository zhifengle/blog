package concurrency

import (
	"fmt"
	"sync"
	"time"
)

func wgDemo() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println("1st goroutine sleeping...")
		time.Sleep(1)
	}()
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println("2nd goroutine sleeping...")
		time.Sleep(2)
	}()
	wg.Wait()
	fmt.Println("All goroutines complete.")
}

func wgHello() {
	hello := func(wg *sync.WaitGroup, id int) {
		defer wg.Done()
		fmt.Printf("Hello from %d!\n", id)
	}
	const numGreeters = 5
	var wg sync.WaitGroup
	wg.Add(numGreeters)
	for i := 0; i < numGreeters; i++ {
		go hello(&wg, i+1)
	}
	wg.Wait()
}

func wgPrintNums() {
	var wg sync.WaitGroup
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			fmt.Println(i)
		}(i)
	}
	wg.Wait()
}

func wgTasks() {
	var wg sync.WaitGroup
	// add 1000 tasks
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func(task int) {
			defer wg.Done()
			fmt.Println("Hello from goroutine: ", task)
		}(i)
		if i%10 == 0 {
			wg.Wait()
		}
	}
	wg.Wait()
}
