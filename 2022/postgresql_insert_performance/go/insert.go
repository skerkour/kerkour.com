package main

import (
	"context"

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

// async fn insert_normalized(db: &DB) {
//     const QUERY: &str = "INSERT INTO normalized (id, type, timestamp, received_at, payload)
//         VALUES ($1, $2, $3, $4, $5)";
//     let stream = stream::iter(0..EXECUTIONS);
//     let base_event = generate_event();

//     stream
//         .for_each_concurrent(CONCURRENCY as usize, |_| {
//             let mut event = base_event.clone();
//             event.id = Uuid::new_v4();
//             async move {
//                 sqlx::query(QUERY)
//                     .bind(&event.id)
//                     .bind(&event.r#type)
//                     .bind(&event.timestamp)
//                     .bind(&event.received_at)
//                     .bind(&event.payload)
//                     .execute(db)
//                     .await
//                     .expect("normalized: inserting event");
//             }
//         })
//         .await;
// }
