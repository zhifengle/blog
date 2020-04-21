package utils

import (
	"fmt"
	"test_go/utils/events"
)

func PrintText(text string) {
	fmt.Println(text)
}

func init() {
	testEvents()
}

func testEvents() {
	e := events.New()

	e.On("log", func(payload ...interface{}) {
		msg := payload[0].(string)
		print(msg)
	})
	e.On("log", func(payload ...interface{}) {
		msg := payload[0].(string)
		print("2: ", msg)
	})

	e.Emit("log", "messages")
}
