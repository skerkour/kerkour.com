package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/jackc/pgx/v4/pgxpool"
)

func main() {
	ctx := context.Background()
	databaseUrl := os.Getenv("DATABASE_URL")
	if databaseUrl == "" {
		log.Fatal("env var DATABASE_URL is missing")
	}

	pool, err := dbConnect(ctx, databaseUrl)
	if err != nil {
		log.Fatal(err)
	}

	defer pool.Close()

}

func dbConnect(ctx context.Context, databaseUrl string) (pool *pgxpool.Pool, err error) {
	config, err := pgxpool.ParseConfig(databaseUrl)
	if err != nil {
		err = fmt.Errorf("parsing database URL: %v", err)
		return
	}

	config.MaxConns = 100

	pool, err = pgxpool.ConnectConfig(ctx, config)
	if err != nil {
		err = fmt.Errorf("Unable to connect to database: %v", err)
		return
	}

	return
}

func dbSetup(ctx context.Context, pool *pgxpool.Pool) (err error) {
	query := `
	CREATE EXTENSION IF NOT EXISTS timescaledb;

    CREATE TABLE IF NOT EXISTS normalized (
        id UUID PRIMARY KEY,
        type TEXT NOT NULL,
        timestamp TIMESTAMPTZ NOT NULL,
        received_at TIMESTAMPTZ NOT NULL,
        payload JSONB NOT NULL
    );

    CREATE TABLE IF NOT EXISTS key_value (
        key UUID PRIMARY KEY,
        value BYTEA NOT NULL
    );

    CREATE TABLE IF NOT EXISTS key_value_compressed (
        key UUID PRIMARY KEY,
        value BYTEA NOT NULL
    );

    CREATE TABLE IF NOT EXISTS timeseries (
        timestamp TIMESTAMPTZ NOT NULL,
        value BYTEA NOT NULL
    );
    CREATE INDEX index_timeseries_on_timestamp ON timeseries (timestamp);


    CREATE TABLE IF NOT EXISTS timeseries_timescale (
        timestamp TIMESTAMPTZ NOT NULL,
        value BYTEA NOT NULL
    );
    SELECT create_hypertable('timeseries_timescale','timestamp');
`
	_, err = pool.Exec(ctx, query)
	return
}

func dbCleanTable(ctx context.Context, pool *pgxpool.Pool, table string) (err error) {
	queryDelete := "DELETE FROM " + table
	queryVacuum := "VACUUM FULL " + table

	_, err = pool.Exec(ctx, queryDelete)
	if err != nil {
		return
	}

	_, err = pool.Exec(ctx, queryVacuum)
	if err != nil {
		return
	}

	return
}
