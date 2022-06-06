package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/google/uuid"
)

const CONCURRENCY = 100
const EXECUTIONS = 100_000
const RUNS = 10

type Event struct {
	ID         uuid.UUID
	Type       string
	Timestamp  time.Time
	ReceivedAt time.Time
	Payload    Payload
}

type Payload struct {
	Something      string
	SomethingElse  []byte
	SomethingElse2 []uint32
}

type KeyValueEvent struct {
	Key   uuid.UUID
	Value []byte
}

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

	// normalized
	fmt.Println("Normalized")
	normalizedResults := make([]time.Duration, RUNS)
	for i := 0; i < RUNS; i++ {
		dbCleanTable(ctx, pool, "normalized")
		start := time.Now()
		errInsert := insertNormalized(ctx, pool)
		end := time.Now()
		normalizedResults[i] = end.Sub(start)
		if errInsert != nil {
			log.Fatal(errInsert)
		}
	}
	fmt.Printf("    results: %v", normalizedResults)
	normalizedMean := durationMean(normalizedResults)
	fmt.Printf("    mean: %v", normalizedMean)

	// for _ in 0..RUNS {
	// 	db::clean_table(&db, "normalized").await;
	// 	let start = Instant::now();
	// 	insert_normalized(&db).await;
	// 	let duration = start.elapsed();
	// 	normalized_results.push(duration);
	// }
	// println!("    results: {:#?}", &normalized_results);
	// let normalized_mean = duration_mean(&normalized_results);
	// println!("    mean: {:?}", &normalized_mean);

	defer pool.Close()
}

func durationMean(results []time.Duration) time.Duration {
	var ret time.Duration

	for _, result := range results {
		ret += result
	}

	return ret / time.Duration(len(results))
}

func generateEvent() Event {
	now := time.Now().UTC()

	payload := Payload{
		Something:      "",
		SomethingElse:  []byte{},
		SomethingElse2: []uint32{},
	}

	return Event{
		ID:         uuid.New(),
		Type:       "some.event.type",
		Timestamp:  now,
		ReceivedAt: now,
		Payload:    payload,
	}
}
