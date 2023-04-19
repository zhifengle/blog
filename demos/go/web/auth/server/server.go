package server

import (
	"context"
	"fmt"
	"net/http"
	"time"
	"web/auth/server/config"
	"web/auth/store"

	"github.com/gin-gonic/gin"
)

var srv *http.Server

type Server struct {
	g *gin.Engine
	// db     *sql.DB
	Store  store.Store
	Config config.Config
}

func NewServer(ctx context.Context, cfg *config.Config) (*Server, error) {
	r := gin.Default()

	s := &Server{
		g:      r,
		Config: *cfg,
	}

	apiGroup := r.Group("/api")

	s.registerAuthRoutes(apiGroup, "somesalt")

	return s, nil
}

func (s *Server) Start(ctx context.Context) error {
	srv = &http.Server{
		Addr:    fmt.Sprintf(":%d", s.Config.Port),
		Handler: s.g,
	}
	fmt.Printf("Version %s has started at :%d\n", s.Config.Version, s.Config.Port)
	return srv.ListenAndServe()
}

func (s *Server) Shutdown(ctx context.Context) error {
	ctx, cancel := context.WithTimeout(ctx, 10*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		return err
	}

	// Close database connection
	// if err := s.db.Close(); err != nil {
	// 	fmt.Printf("failed to close database, error: %v\n", err)
	// }

	fmt.Printf("server stopped properly\n")
	return nil
}
