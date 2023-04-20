package main

import (
	"io"
	"log"
	"net"
	"strconv"
)

func handleConnection(conn net.Conn) {
	defer conn.Close()

	// Read initial greeting
	buf := make([]byte, 256)
	n, err := conn.Read(buf)
	if err != nil {
		log.Println("Error reading greeting:", err)
		return
	}

	// Check version
	if buf[0] != 5 {
		log.Println("Unsupported SOCKS version:", buf[0])
		return
	}

	// Send supported version
	conn.Write([]byte{5, 0})

	// Read request
	n, err = conn.Read(buf)
	if err != nil {
		log.Println("Error reading request:", err)
		return
	}

	// Check command
	if buf[1] != 1 {
		log.Println("Unsupported SOCKS command:", buf[1])
		return
	}

	// Extract host and port
	var host string
	switch buf[3] {
	case 1:
		host = net.IPv4(buf[4], buf[5], buf[6], buf[7]).String()
	case 3:
		host = string(buf[5 : n-2])
	case 4:
		host = net.IP(buf[4:8]).String()
	}

	port := strconv.Itoa(int(buf[n-2])<<8 | int(buf[n-1]))

	// Dial target host
	targetAddr := net.JoinHostPort(host, port)
	targetConn, err := net.Dial("tcp", targetAddr)
	if err != nil {
		log.Println("Error dialing target host:", err)
		return
	}
	defer targetConn.Close()

	// Send success response
	conn.Write([]byte{5, 0, 0, 1, 0, 0, 0, 0, 0, 0})

	// Relay traffic between client and target
	go func() {
		_, err := io.Copy(targetConn, conn)
		if err != nil {
			log.Println("Error copying to target:", err)
		}
	}()
	_, err = io.Copy(conn, targetConn)
	if err != nil {
		log.Println("Error copying from target:", err)
	}
}

func main() {
	listener, err := net.Listen("tcp", ":1080")
	if err != nil {
		log.Fatal("Error listening:", err)
	}
	defer listener.Close()
	log.Println("SOCKS server listening on", listener.Addr())

	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Println("Error accepting connection:", err)
			continue
		}
		go handleConnection(conn)
	}
}
