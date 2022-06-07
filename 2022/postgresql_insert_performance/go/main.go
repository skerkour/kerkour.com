package main

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"os"
	"strings"
	"time"

	"github.com/google/uuid"
)

const CONCURRENCY = 100
const EXECUTIONS = 30_000
const RUNS = 7

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

	fmt.Println("\n------------------------------------------\n")

	// time series
	fmt.Println("Timeseries")
	timeseriesResults := make([]time.Duration, RUNS)
	for i := 0; i < RUNS; i++ {
		dbCleanTable(ctx, pool, "timeseries")
		start := time.Now()
		errInsert := insertTimeSeries(ctx, pool)
		end := time.Now()
		timeseriesResults[i] = end.Sub(start)
		if errInsert != nil {
			log.Fatal(errInsert)
		}
	}
	fmt.Printf("    results: %v", timeseriesResults)
	timeseriesMean := durationMean(timeseriesResults)
	fmt.Printf("    mean: %v", timeseriesMean)
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
		Something:      RandString(500),
		SomethingElse:  make([]byte, 500),
		SomethingElse2: make([]uint32, 500),
	}

	return Event{
		ID:         uuid.New(),
		Type:       "some.event.type",
		Timestamp:  now,
		ReceivedAt: now,
		Payload:    payload,
	}
}

const letterBytes = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
const (
	letterIdxBits = 6                    // 6 bits to represent a letter index
	letterIdxMask = 1<<letterIdxBits - 1 // All 1-bits, as many as letterIdxBits
	letterIdxMax  = 63 / letterIdxBits   // # of letter indices fitting in 63 bits
)

func RandString(n int) string {
	var src = rand.NewSource(time.Now().UnixNano())
	sb := strings.Builder{}
	sb.Grow(n)
	// A src.Int63() generates 63 random bits, enough for letterIdxMax characters!
	for i, cache, remain := n-1, src.Int63(), letterIdxMax; i >= 0; {
		if remain == 0 {
			cache, remain = src.Int63(), letterIdxMax
		}
		if idx := int(cache & letterIdxMask); idx < len(letterBytes) {
			sb.WriteByte(letterBytes[idx])
			i--
		}
		cache >>= letterIdxBits
		remain--
	}

	return sb.String()
}
