package main

import (
	"encoding/json"
	"net/http"
)

// 响应 JSON
type Person struct {
	Name  string `json:"name"`
	Email string `json:"email"`
	Age   int    `json:"age"`
}

func getPerson(w http.ResponseWriter, r *http.Request) {
	// Create a Person struct
	person := Person{
		Name:  "John Doe",
		Email: "johndoe@example.com",
		Age:   30,
	}

	// Set the Content-Type header to application/json
	w.Header().Set("Content-Type", "application/json")

	// Encode the Person struct as JSON and write it to the response
	json.NewEncoder(w).Encode(person)
}

func addPerson(w http.ResponseWriter, r *http.Request) {
	// http.Error(w, "Person not found", http.StatusNotFound)

	// Only allow POST requests
	if r.Method != "POST" {
		http.Error(w, http.StatusText(http.StatusBadRequest), http.StatusBadRequest)
		return
	}

	// Parse the request body as JSON
	var person Person
	err := json.NewDecoder(r.Body).Decode(&person)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Do something with the person data (e.g. save to database)

	// Send a response back to the client
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(person)
}
