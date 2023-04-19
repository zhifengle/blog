package common

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
)

type UserServer interface {
	Start(ctx context.Context) error
	Shutdown(ctx context.Context) error
}

func StartServer(s UserServer) {
	ctx, cancel := context.WithCancel(context.Background())

	c := make(chan os.Signal, 1)
	// Trigger graceful shutdown on SIGINT or SIGTERM.
	// The default signal sent by the `kill` command is SIGTERM,
	// which is taken as the graceful shutdown signal for many systems, eg., Kubernetes, Gunicorn.
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		sig := <-c
		fmt.Printf("%s received.\n", sig.String())
		err := s.Shutdown(ctx)
		if err != nil {
			fmt.Printf("failed to shutdown server, error: %+v\n", err)
		}
		cancel()
	}()

	if err := s.Start(ctx); err != nil {
		if err != http.ErrServerClosed {
			cancel()
			log.Fatalf("failed to start server, error: %+v\n", err)
		}
	}

	// Wait for CTRL-C.
	<-ctx.Done()
}
