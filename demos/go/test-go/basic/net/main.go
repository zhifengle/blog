package main

import (
	"fmt"
	"net"
)

func main() {
	ip := &net.IPAddr{IP: net.IPv4(39, 156, 66, 18)}
	recv, totalDlay := checkConnection(ip)

	fmt.Println("recv: ", recv, totalDlay)
}
