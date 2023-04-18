package main

import (
	"github.com/gin-contrib/sessions"
	"github.com/gin-contrib/sessions/cookie"
	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()

	store := cookie.NewStore([]byte("secret"))
	router.Use(sessions.Sessions("mysession", store))

	router.POST("/login", loginHandler)
	router.GET("/logout", logoutHandler)

	router.GET("/protected", protectedHandler)
	router.Run(":7080")
}

func loginHandler(c *gin.Context) {
	session := sessions.Default(c)
	username := c.PostForm("username")
	password := c.PostForm("password")

	// authenticate the user
	if username == "user" && password == "password" {
		session.Set("username", username)
		session.Save()

		c.JSON(200, gin.H{"message": "Logged in successfully"})
	} else {
		c.JSON(401, gin.H{"error": "Invalid username or password"})
	}
}

func logoutHandler(c *gin.Context) {
	session := sessions.Default(c)
	session.Clear()
	session.Save()

	c.JSON(200, gin.H{"message": "Logged out successfully"})
}

func protectedHandler(c *gin.Context) {
	session := sessions.Default(c)

	// check if the user is authenticated
	username := session.Get("username")
	if username == nil {
		c.JSON(401, gin.H{"error": "Unauthorized"})
		c.Abort()
		return
	}

	c.JSON(200, gin.H{"message": "Protected endpoint reached"})
}
