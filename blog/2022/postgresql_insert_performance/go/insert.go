package main

import (
	"context"
	"encoding/json"
	"log"
	"time"

	"github.com/DataDog/zstd"
	"github.com/alitto/pond"
	"github.com/golang/snappy"
	"github.com/google/uuid"
	"github.com/jackc/pgx/v4/pgxpool"
)

func insertNormalized(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO normalized (id, type, timestamp, received_at, payload)
	         VALUES ($1, $2, $3, $4, $5)`
	baseEvent := generateEvent()
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			event := baseEvent
			event.ID = uuid.New()
			_, jobErr := pool.Exec(ctx, query, event.ID, event.Type, event.Timestamp, event.ReceivedAt, event.Payload)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}

func insertKeyValue(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO key_value (key, value)
	       VALUES ($1, $2)`
	baseEvent := generateEvent()
	jsonPayload, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	keyValueEvent := KeyValueEvent{
		Key:   baseEvent.ID,
		Value: jsonPayload,
	}
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			event := keyValueEvent
			event.Key = uuid.New()
			_, jobErr := pool.Exec(ctx, query, event.Key, event.Value)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}

func insertKeyValueCompressedZstd(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO key_value_compressed_zstd (key, value)
	         VALUES ($1, $2)`
	baseEvent := generateEvent()
	jsonPayload, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	keyValueEvent := KeyValueEvent{
		Key:   baseEvent.ID,
		Value: []byte{},
	}
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			event := keyValueEvent
			event.Key = uuid.New()
			comrpessedPayload, jobErr := zstd.CompressLevel(nil, jsonPayload, 2)
			if err != nil {
				log.Fatal(jobErr)
			}
			event.Value = comrpessedPayload
			_, jobErr = pool.Exec(ctx, query, event.Key, event.Value)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}

func insertKeyValueCompressedSnappy(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO key_value_compressed_snappy (key, value)
	         VALUES ($1, $2)`
	baseEvent := generateEvent()
	jsonPayload, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	keyValueEvent := KeyValueEvent{
		Key:   baseEvent.ID,
		Value: []byte{},
	}
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			event := keyValueEvent
			event.Key = uuid.New()
			comrpessedPayload := snappy.Encode(nil, jsonPayload)
			event.Value = comrpessedPayload
			_, jobErr := pool.Exec(ctx, query, event.Key, event.Value)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}

func insertTimeSeries(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO timeseries (timestamp, value)
	        VALUES ($1, $2)`
	baseEvent := generateEvent()
	jsonEvent, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			timestamp := time.Now()
			_, jobErr := pool.Exec(ctx, query, timestamp, jsonEvent)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}

func insertTimeSeriesSnappy(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO timeseries (timestamp, value)
	        VALUES ($1, $2)`
	baseEvent := generateEvent()
	jsonEvent, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			timestamp := time.Now()
			comrpessedPayload := snappy.Encode(nil, jsonEvent)
			_, jobErr := pool.Exec(ctx, query, timestamp, comrpessedPayload)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}

func insertTimeSeriesTimescale(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO timeseries_timescale (timestamp, value)
	        VALUES ($1, $2)`
	baseEvent := generateEvent()
	jsonEvent, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			timestamp := time.Now()
			_, jobErr := pool.Exec(ctx, query, timestamp, jsonEvent)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}

func insertTimeSeriesTimescaleSnappy(ctx context.Context, pool *pgxpool.Pool) (err error) {
	const query = `INSERT INTO timeseries_timescale (timestamp, value)
	        VALUES ($1, $2)`
	baseEvent := generateEvent()
	jsonEvent, err := json.Marshal(baseEvent)
	if err != nil {
		return
	}
	workers := pond.New(CONCURRENCY, 0, pond.MinWorkers(CONCURRENCY))

	for i := 0; i < EXECUTIONS; i++ {
		workers.Submit(func() {
			timestamp := time.Now()
			comrpessedPayload := snappy.Encode(nil, jsonEvent)
			_, jobErr := pool.Exec(ctx, query, timestamp, comrpessedPayload)
			if jobErr != nil {
				log.Fatal(jobErr)
			}
		})
	}

	workers.StopAndWait()
	return
}
