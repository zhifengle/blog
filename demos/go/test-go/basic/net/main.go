package main

import (
	"fmt"
	"net"
	"time"
)

const (
	tcpConnectTimeout = time.Second * 1
	maxRoutine        = 1000
	defaultRoutines   = 200
	defaultPort       = 443
	defaultPingTimes  = 4
)

var (
	Routines      = defaultRoutines
	TCPPort   int = defaultPort
	PingTimes int = defaultPingTimes
)

func main() {
	ip := &net.IPAddr{IP: net.IPv4(39, 156, 66, 18)}
	recv, totalDlay := checkConnection(ip)

	fmt.Println("recv: ", recv, totalDlay)
}

func tcping(ip *net.IPAddr) (bool, time.Duration) {
	startTime := time.Now()
	var fullAddress string
	fullAddress = fmt.Sprintf("%s:%d", ip.String(), TCPPort)
	conn, err := net.DialTimeout("tcp", fullAddress, tcpConnectTimeout)
	if err != nil {
		return false, 0
	}
	defer conn.Close()
	duration := time.Since(startTime)
	return true, duration
}

func checkConnection(ip *net.IPAddr) (recv int, totalDelay time.Duration) {
	for i := 0; i < PingTimes; i++ {
		if ok, delay := tcping(ip); ok {
			recv++
			totalDelay += delay
		}
	}
	return recv, totalDelay
}
