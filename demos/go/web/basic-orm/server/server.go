package server

import (
	"context"
	"fmt"
	"net/http"
	"time"
	"web/basic-orm/server/profile"
	"web/basic-orm/store"
	"web/common"

	"github.com/gin-gonic/gin"
)

var srv *http.Server

type Server struct {
	g       *gin.Engine
	Store   *store.Store
	Profile *profile.Profile
}

func NewServer(ctx context.Context, cfg *profile.Profile, store *store.Store) (*Server, error) {
	r := gin.Default()
	s := &Server{
		g:       r,
		Profile: cfg,
		Store:   store,
	}
	secret := cfg.Secret
	if secret == "" {
		secret = "secret"
	}

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{"code": 0, "msg": "ping"})
	})
	r.GET("/pong", func(c *gin.Context) {
		c.JSON(200, gin.H{"code": 0, "msg": "pong"})
	})
	r.Use(ErrorHandler())

	apiGroup := r.Group("/api")

	s.registerAuthRoutes(apiGroup, secret)
	s.registerUserRoutes(apiGroup)
	return s, nil
}

func (s *Server) Start(ctx context.Context) error {
	srv = &http.Server{
		Addr:    fmt.Sprintf(":%d", s.Profile.Port),
		Handler: s.g,
	}
	fmt.Printf("Version %s has started at :%d\n", s.Profile.Version, s.Profile.Port)
	return srv.ListenAndServe()
}

func (s *Server) Shutdown(ctx context.Context) error {
	ctx, cancel := context.WithTimeout(ctx, 10*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		return err
	}

	if s.Store != nil {
		// Close database connection
		if err := s.Store.Close(); err != nil {
			fmt.Printf("failed to close database, error: %v\n", err)
		}
	}

	fmt.Printf("server stopped properly\n")
	return nil
}

func (s *Server) IsPublic(c *gin.Context) bool {
	if common.HasPrefixes(c.Request.URL.Path, "/api/auth", "/ping") {
		return true
	}
	return false
}
