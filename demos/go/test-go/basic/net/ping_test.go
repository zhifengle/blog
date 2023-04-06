package main

import "testing"

func TestPing(t *testing.T) {
	pingBaidu()
	tcpingSite("www.bing.com")
	tcpingSite("39.156.66.18")
}
