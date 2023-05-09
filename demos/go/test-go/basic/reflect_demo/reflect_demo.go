package main

import (
	"fmt"
	"reflect"
)

type Person struct {
	Name    string
	Age     int
	Address string
}

func main() {
	// Create an instance of Person struct
	p1 := Person{Name: "John", Age: 30, Address: "123 Main St"}

	// Call a function to convert struct to map
	p1Map := structToMap(p1)

	// Print the result
	fmt.Println(p1Map)
}

func structToMap(obj interface{}) map[string]string {
	// Initialize an empty map
	result := make(map[string]string)

	// Get the reflect value of the input object
	objValue := reflect.ValueOf(obj)

	// If the input is not a struct, return an empty map
	if objValue.Kind() != reflect.Struct {
		return result
	}

	// Get the type of the input object
	objType := objValue.Type()

	// Iterate over the fields of the struct
	for i := 0; i < objValue.NumField(); i++ {
		// Get the reflect value and type of the current field
		fieldValue := objValue.Field(i)
		fieldType := objType.Field(i)

		// If the field is not exportable, skip it
		if !fieldValue.CanInterface() {
			continue
		}

		// Convert the reflect value of the field to string and add it to the result map
		result[fieldType.Name] = fmt.Sprintf("%v", fieldValue.Interface())
	}

	return result
}
