package main

import (
	"fmt"
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"
)

func main() {
	targetUrl := "http://localhost:8080"
	u, _ := url.Parse(targetUrl)
	// Create a new reverse proxy.
	reverseProxy := httputil.NewSingleHostReverseProxy(u)

	// Create a handler function that will be used for incoming requests
	handler := http.HandlerFunc(func(rw http.ResponseWriter, req *http.Request) {
		fmt.Printf("Forwarding request to %s\n", targetUrl)
		reverseProxy.ServeHTTP(rw, req)
	})

	// Start the reverse proxy server
	srv := &http.Server{
		Addr:    ":8081",
		Handler: handler,
	}
	log.Fatal(srv.ListenAndServe())
}
