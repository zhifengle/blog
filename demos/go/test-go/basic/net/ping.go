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

func tcpingSite(host string) {
	ipAddr, err := net.ResolveIPAddr("ip", host)
	if err != nil {
		fmt.Println("error resolving IP address: ", err)
		return
	}
	ok, delay := tcping(ipAddr)
	if !ok {
		fmt.Println("tcping failed")
	}
	fmt.Printf("Ping succeeded: %s (%s)\n", ipAddr.String(), delay)
}

func tcping(ip *net.IPAddr) (bool, time.Duration) {
	startTime := time.Now()
	fullAddress := fmt.Sprintf("%s:%d", ip.String(), TCPPort)
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

// icmp ping baidu
func pingBaidu() {
	ipAddr, err := net.ResolveIPAddr("ip", "www.baidu.com")
	if err != nil {
		fmt.Println("error resolving IP address: ", err)
		return
	}
	start := time.Now()
	conn, err := net.DialIP("ip4:icmp", nil, ipAddr)
	elapsed := time.Since(start)
	if err != nil {
		fmt.Println("error pinging IP address: ", err)
		return
	}
	fmt.Printf("Ping succeeded: %s (%s)\n", ipAddr.String(), elapsed)
	defer conn.Close()
}
