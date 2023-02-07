package main

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"os/exec"

	"github.com/gin-gonic/gin"
)

type UserCmd struct {
	Name string   `form:"name" json:"name" xml:"name"  binding:"required"`
	Args []string `form:"args" json:"args" xml:"args"`
}

func getCmdPath(name string) string {
	content, err := ioutil.ReadFile("config.json")
	if err != nil {
		log.Fatal("Error when opening file: ", err)
	}
	config := make(map[string]string)
	err = json.Unmarshal(content, &config)
	if err != nil {
		log.Fatal("Error during Unmarshal(): ", err)
	}
	log.Printf("origin: %v\n", config)
	return config[name]
}

func main() {
	r := gin.Default()
	v1 := r.Group("/v1")
	{
		v1.POST("/cmd", cmdHandler)
	}
	r.Run() // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}

func cmdHandler(c *gin.Context) {
	var userCmd UserCmd
	//userCmd := make(map[string]interface{}) //注意该结构接受的内容
	if err := c.ShouldBindJSON(&userCmd); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	path := getCmdPath(userCmd.Name)
	if path == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid name"})
		return
	}
	cmd := exec.Command(path, userCmd.Args...)
	out, err := cmd.CombinedOutput()
	if err != nil {
		// @TODO err string
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	c.JSON(http.StatusOK, gin.H{
		"data": string(out),
	})
}
