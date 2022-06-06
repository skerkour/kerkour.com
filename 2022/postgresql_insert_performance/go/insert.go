package main

import (
	"context"
	"encoding/json"
	"time"

	"github.com/DataDog/zstd"
	"github.com/golang/snappy"
	"github.com/google/uuid"
	"github.com/jackc/pgx/v4/pgxpool"
)

func insertNormalized(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO normalized (id, type, timestamp, received_at, payload)
	         VALUES ($1, $2, $3, $4, $5)`
	baseEvent := generateEvent()
	jobs := make(chan Event, CONCURRENCY)
	results := make(chan error, CONCURRENCY)

	worker := func(ctx context.Context, pool *pgxpool.Pool, jobs <-chan Event, results chan<- error) {
		for job := range jobs {
			event := job
			event.ID = uuid.New()
			_, jobErr := pool.Exec(ctx, query, event.ID, event.Type, event.Timestamp, event.ReceivedAt, event.Payload)
			results <- jobErr
		}
	}

	for w := 0; w < CONCURRENCY; w++ {
		go worker(ctx, pool, jobs, results)
	}

	for i := 0; i < EXECUTIONS; i++ {
		jobs <- baseEvent
	}
	close(jobs)

	for i := 0; i < EXECUTIONS; i++ {
		err = <-results
		if err != nil {
			return
		}
	}

	return
}

func insertKeyValue(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO key_value (key, value)
	       VALUES ($1, $2)`
	baseEvent := generateEvent()
	jobs := make(chan KeyValueEvent, CONCURRENCY)
	results := make(chan error, CONCURRENCY)
	jsonPayload, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	keyValueEvent := KeyValueEvent{
		Key:   baseEvent.ID,
		Value: jsonPayload,
	}

	worker := func(ctx context.Context, pool *pgxpool.Pool, jobs <-chan KeyValueEvent, results chan<- error) {
		for job := range jobs {
			event := job
			event.Key = uuid.New()
			_, jobErr := pool.Exec(ctx, query, event.Key, event.Value)
			results <- jobErr
		}
	}

	for w := 0; w < CONCURRENCY; w++ {
		go worker(ctx, pool, jobs, results)
	}

	for i := 0; i < EXECUTIONS; i++ {
		jobs <- keyValueEvent
	}
	close(jobs)

	for i := 0; i < EXECUTIONS; i++ {
		err = <-results
		if err != nil {
			return
		}
	}

	return
}

func insertKeyValueCompressed(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO key_value_compressed_zstd (key, value)
	         VALUES ($1, $2)`
	baseEvent := generateEvent()
	jobs := make(chan KeyValueEvent, CONCURRENCY)
	results := make(chan error, CONCURRENCY)
	jsonPayload, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	keyValueEvent := KeyValueEvent{
		Key:   baseEvent.ID,
		Value: []byte{},
	}

	worker := func(ctx context.Context, pool *pgxpool.Pool, jobs <-chan KeyValueEvent, results chan<- error) {
		for job := range jobs {
			event := job
			event.Key = uuid.New()
			comrpessedPayload, jobErr := zstd.CompressLevel(nil, jsonPayload, 2)
			if err != nil {
				results <- jobErr
				return
			}
			event.Value = comrpessedPayload
			_, jobErr = pool.Exec(ctx, query, event.Key, event.Value)
			results <- jobErr
		}
	}

	for w := 0; w < CONCURRENCY; w++ {
		go worker(ctx, pool, jobs, results)
	}

	for i := 0; i < EXECUTIONS; i++ {
		jobs <- keyValueEvent
	}
	close(jobs)

	for i := 0; i < EXECUTIONS; i++ {
		err = <-results
		if err != nil {
			return
		}
	}

	return
}

func insertKeyValueCompressedSnappy(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO key_value_compressed_snappy (key, value)
	         VALUES ($1, $2)`
	baseEvent := generateEvent()
	jobs := make(chan KeyValueEvent, CONCURRENCY)
	results := make(chan error, CONCURRENCY)
	jsonPayload, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	keyValueEvent := KeyValueEvent{
		Key:   baseEvent.ID,
		Value: []byte{},
	}

	worker := func(ctx context.Context, pool *pgxpool.Pool, jobs <-chan KeyValueEvent, results chan<- error) {
		for job := range jobs {
			event := job
			event.Key = uuid.New()
			comrpessedPayload := snappy.Encode(nil, jsonPayload)
			event.Value = comrpessedPayload
			_, jobErr := pool.Exec(ctx, query, event.Key, event.Value)
			results <- jobErr
		}
	}

	for w := 0; w < CONCURRENCY; w++ {
		go worker(ctx, pool, jobs, results)
	}

	for i := 0; i < EXECUTIONS; i++ {
		jobs <- keyValueEvent
	}
	close(jobs)

	for i := 0; i < EXECUTIONS; i++ {
		err = <-results
		if err != nil {
			return
		}
	}

	return
}

func insertTimeSeries(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO timeseries (timestamp, value)
	        VALUES ($1, $2)`
	baseEvent := generateEvent()
	jobs := make(chan Event, CONCURRENCY)
	results := make(chan error, CONCURRENCY)
	jsonEvent, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}

	worker := func(ctx context.Context, pool *pgxpool.Pool, jobs <-chan Event, results chan<- error) {
		for range jobs {
			timestamp := time.Now()
			_, jobErr := pool.Exec(ctx, query, timestamp, jsonEvent)
			results <- jobErr
		}
	}

	for w := 0; w < CONCURRENCY; w++ {
		go worker(ctx, pool, jobs, results)
	}

	for i := 0; i < EXECUTIONS; i++ {
		jobs <- baseEvent
	}
	close(jobs)

	for i := 0; i < EXECUTIONS; i++ {
		err = <-results
		if err != nil {
			return
		}
	}

	return
}
