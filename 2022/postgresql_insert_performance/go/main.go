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
	defer pool.Close()

	err = dbSetup(ctx, pool)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("DB Setup")

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
		fmt.Printf("%d,", i)
	}
	fmt.Println("")
	fmt.Printf("    results: %v", normalizedResults)
	normalizedMean := durationMean(normalizedResults)
	fmt.Printf("    mean: %v", normalizedMean)

	fmt.Println("\n------------------------------------------\n")

	// normalized
	fmt.Println("Key Value")
	keyValueResults := make([]time.Duration, RUNS)
	for i := 0; i < RUNS; i++ {
		dbCleanTable(ctx, pool, "key_value")
		start := time.Now()
		errInsert := insertKeyValue(ctx, pool)
		end := time.Now()
		keyValueResults[i] = end.Sub(start)
		if errInsert != nil {
			log.Fatal(errInsert)
		}
	}
	fmt.Printf("    results: %v", keyValueResults)
	keyValueMean := durationMean(keyValueResults)
	fmt.Printf("    mean: %v", keyValueMean)

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
