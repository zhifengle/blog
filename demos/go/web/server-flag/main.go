package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
)

func main() {
	// Define and parse command line flag for port number
	port := flag.Int("port", 8080, "port number")
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s [-port port_number]\n", os.Args[0])
		flag.PrintDefaults()
	}
	flag.Parse()

	// Register a handler for the root endpoint
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, World!")
	})

	// Start the HTTP server on the specified port
	log.Printf("Starting server on port %d", *port)
	err := http.ListenAndServe(fmt.Sprintf(":%d", *port), nil)
	if err != nil {
		log.Fatal(err)
	}
}
