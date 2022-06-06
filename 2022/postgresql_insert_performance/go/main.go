package main

import (
	"context"
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
