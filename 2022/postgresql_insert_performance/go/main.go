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
	"github.com/jackc/pgx/v4/pgxpool"
)

const CONCURRENCY = 100
const EXECUTIONS = 50_000
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

	run(ctx, "Normalized", RUNS, pool, "normalized", insertNormalized)

	fmt.Println("\n------------------------------------------\n")

	run(ctx, "Key Value", RUNS, pool, "key_value", insertKeyValue)

	fmt.Println("\n------------------------------------------\n")

	run(ctx, "Key Value ZSTD", RUNS, pool, "key_value_compressed_zstd", insertKeyValueCompressedZstd)

	fmt.Println("\n------------------------------------------\n")

	run(ctx, "Key Value Snappy", RUNS, pool, "key_value_compressed_snappy", insertKeyValueCompressedSnappy)

	fmt.Println("\n------------------------------------------\n")

	run(ctx, "Timeseries", RUNS, pool, "timeseries", insertTimeSeries)

	fmt.Println("\n------------------------------------------\n")

	run(ctx, "Timeseries Snappy", RUNS, pool, "timeseries", insertTimeSeriesSnappy)

	fmt.Println("\n------------------------------------------\n")

	run(ctx, "Timeseries Timescale", RUNS, pool, "timeseries", insertTimeSeriesTimescale)

	fmt.Println("\n------------------------------------------\n")

	run(ctx, "Timeseries Timescale Snappy", RUNS, pool, "timeseries", insertTimeSeriesTimescaleSnappy)
}

func run(ctx context.Context, name string, runs uint, pool *pgxpool.Pool, table string, fn func(context.Context, *pgxpool.Pool) error) {
	fmt.Println(name)
	results := make([]time.Duration, runs)
	var err error

	for i := 0; i < RUNS; i++ {
		dbCleanTable(ctx, pool, table)
		start := time.Now()
		err = fn(ctx, pool)
		end := time.Now()
		results[i] = end.Sub(start)
		if err != nil {
			log.Fatal(err)
		}
	}
	fmt.Printf("    results: %v\n", results)
	mean := durationMean(results)
	reqsPerSec := EXECUTIONS / mean.Seconds()
	fmt.Printf("    mean: %v (%.2f reqs /s)\n", mean, reqsPerSec)
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
