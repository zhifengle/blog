package main

import (
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()

	// Attach the middleware to the router
	r.Use(operationLogger())

	// Define a route
	r.GET("/hello", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "Hello, world!",
		})
	})

	// Start the server
	if err := r.Run(":8080"); err != nil {
		log.Fatal("failed to start server: ", err)
	}
}

// operationLogger is a middleware that logs user operations
func operationLogger() gin.HandlerFunc {
	return func(c *gin.Context) {
		// Record the start time of the request
		startTime := time.Now()

		// Call the next middleware/handler
		c.Next()

		// Record the end time of the request and calculate the duration
		endTime := time.Now()
		duration := endTime.Sub(startTime)

		// Log the operation details
		log.Printf("[%s]%s %s %s %d %s %s",
			endTime.Format("2006/01/02 - 15:04:05"),
			c.ClientIP(),
			c.Request.Method,
			c.Request.URL.Path,
			c.Writer.Status(),
			c.Request.UserAgent(),
			duration,
		)
	}
}
