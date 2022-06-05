package main

import (
	"context"
	"log"
	"os"

	"github.com/jackc/pgx/v4/pgxpool"
)

func main() {
	databaseUrl := os.Getenv("DATABASE_URL")
	if databaseUrl == "" {
		log.Fatal("env var DATABASE_URL is missing")
	}

	ctx := context.Background()

	config, err := pgxpool.ParseConfig(databaseUrl)
	if err != nil {
		log.Fatalf("parsing database URL: %v", err)
	}

	config.MaxConns = 100

	dbpool, err := pgxpool.ConnectConfig(ctx, config)
	if err != nil {
		log.Fatalf("Unable to connect to database: %v", err)
	}
	defer dbpool.Close()

}
