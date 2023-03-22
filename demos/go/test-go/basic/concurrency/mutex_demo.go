package concurrency

import (
	"fmt"
	"sync"
)

func mutexDemo() {
	var m sync.Mutex
	m.Lock()

	go func() {
		fmt.Println("1st goroutine sleeping...")
		m.Unlock()
	}()
	m.Lock()
}
