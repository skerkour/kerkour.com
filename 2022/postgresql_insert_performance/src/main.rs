use chrono::{DateTime, Utc};
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::Json, Executor, Pool, Postgres};
use std::time::{Duration, Instant};
use uuid::Uuid;

type DB = Pool<Postgres>;
const CONCURRENCY: u32 = 100;
const EXECUTIONS: u64 = 100_000;

#[derive(sqlx::FromRow, Serialize, Debug, Clone, Deserialize)]
struct Event {
    id: uuid::Uuid,
    r#type: String,
    timestamp: DateTime<Utc>,
    received_at: DateTime<Utc>,
    payload: Json<Payload>,
}

#[derive(sqlx::FromRow, Serialize, Debug, Clone, Deserialize)]
struct KeyValueEvent {
    key: uuid::Uuid,
    value: Vec<u8>,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
struct Payload {
    something: String,
    something_else: Vec<u8>,
    something_else2: Vec<u32>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env is missing");

    let db = db_connect(&database_url).await?;

    db_setup(&db).await?;

    // normalized
    let mut normalized_results = Vec::with_capacity(10);
    for _ in 0..10 {
        let start = Instant::now();
        insert_normalized(&db).await;
        let duration = start.elapsed();
        normalized_results.push(duration);
    }
    clean_table(&db, "normalized").await;
    println!("Normalized: {:#?}", &normalized_results);
    let normalized_mean = duration_mean(&normalized_results);
    println!("    mean: {:?}", &normalized_mean);

    // key_value
    let mut key_value_results = Vec::with_capacity(10);
    for _ in 0..10 {
        let start = Instant::now();
        insert_key_value(&db).await;
        let duration = start.elapsed();
        key_value_results.push(duration);
    }
    clean_table(&db, "key_value").await;
    println!("Key Value: {:#?}", &normalized_results);
    let key_value_mean = duration_mean(&key_value_results);
    println!("    mean: {:?}", &key_value_mean);

    Ok(())
}

fn duration_mean(results: &[Duration]) -> Duration {
    let result = results.iter().fold(Duration::ZERO, |acc, x| acc + *x);
    result / results.len() as u32
}

async fn db_connect(database_url: &str) -> Result<DB, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(CONCURRENCY)
        .connect_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await?;
    Ok(pool)
}

fn generate_event() -> Event {
    let now = Utc::now();
    let payload = Payload {
        something: String::new(),
        something_else: Vec::new(),
        something_else2: Vec::new(),
    };

    Event {
        id: Uuid::new_v4(),
        r#type: "some.event.type".to_string(),
        timestamp: now,
        received_at: now,
        payload: Json(payload),
    }
}

async fn db_setup(db: &DB) -> Result<(), anyhow::Error> {
    db.execute(
        "
    CREATE EXTENSION IF NOT EXISTS timescaledb;

    CREATE TABLE IF NOT EXISTS normalized (
        id UUID PRIMARY KEY,
        type TEXT NOT NULL,
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        received_at TIMESTAMP WITH TIME ZONE NOT NULL,
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
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        value BYTEA NOT NULL
    );
    CREATE INDEX index_timeseries_on_timestamp ON timeseries (timestamp);


    CREATE TABLE IF NOT EXISTS timeseries_timescale (
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        value BYTEA NOT NULL
    );
    SELECT create_hypertable('timeseries_timescale','timestamp');

    ",
    )
    .await?;
    Ok(())
}

async fn clean_table(db: &DB, table: &str) {
    let query_delete = "DELETE FROM ".to_string() + table;
    let query_vacuum = "VACUUM FULL ".to_string() + table;

    sqlx::query(&query_delete)
        .execute(db)
        .await
        .expect("clean_table: deleting events");

    sqlx::query(&query_vacuum)
        .execute(db)
        .await
        .expect("clean_table: vacuuming table");
}

async fn insert_normalized(db: &DB) {
    const QUERY: &str = "INSERT INTO normalized (id, type, timestamp, received_at, payload)
        VALUES ($1, $2, $3, $4, $5)";
    let stream = stream::iter(0..EXECUTIONS);
    let base_event = generate_event();

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| {
            let mut event = base_event.clone();
            event.id = Uuid::new_v4();
            async move {
                sqlx::query(QUERY)
                    .bind(&event.id)
                    .bind(&event.r#type)
                    .bind(&event.timestamp)
                    .bind(&event.received_at)
                    .bind(&event.payload)
                    .execute(db)
                    .await
                    .expect("normalized: inserting event");
            }
        })
        .await;
}

async fn insert_key_value(db: &DB) {
    const QUERY: &str = "INSERT INTO key_value (key, value)
        VALUES ($1, $2)";
    let stream = stream::iter(0..EXECUTIONS);
    let base_event = generate_event();
    let key_value_event = KeyValueEvent {
        key: base_event.id,
        value: serde_json::to_vec(&base_event).expect("key_value: serializing event"),
    };

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| {
            let mut event = key_value_event.clone();
            event.key = Uuid::new_v4();
            async move {
                sqlx::query(QUERY)
                    .bind(&event.key)
                    .bind(&event.value)
                    .execute(db)
                    .await
                    .expect("key_value: inserting event");
            }
        })
        .await;
}

async fn insert_key_value_compressed(db: &DB) {
    const QUERY: &str = "INSERT INTO key_value_compressed (key, value)
        VALUES ($1, $2)";
    let stream = stream::iter(0..EXECUTIONS);
    let base_event = generate_event();
    let key_value_event = KeyValueEvent {
        key: base_event.id,
        value: serde_json::to_vec(&base_event).expect("key_value_compressed: serializing event"),
    };

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| {
            let mut event = key_value_event.clone();
            event.key = Uuid::new_v4();
            async move {
                event.value = tokio::task::spawn_blocking(move || {
                    zstd::bulk::compress(&event.value, 2)
                        .expect("key_value_compressed :compressing event")
                })
                .await
                .expect("key_value_compressed: awaiting for tokio::spawn_blocking");

                sqlx::query(QUERY)
                    .bind(&event.key)
                    .bind(&event.value)
                    .execute(db)
                    .await
                    .expect("key_value_compressed: inserting event");
            }
        })
        .await;
}

async fn insert_timeseries(db: &DB) {
    const QUERY: &str = "INSERT INTO timeseries (timestamp, value)
        VALUES ($1, $2)";
    let stream = stream::iter(0..EXECUTIONS);
    let base_event = generate_event();
    let value = serde_json::to_vec(&base_event).expect("timeseries: serializing event");

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| async {
            sqlx::query(QUERY)
                .bind(&base_event.timestamp)
                .bind(&value)
                .execute(db)
                .await
                .expect("timeseries: inserting event");
        })
        .await;
}
