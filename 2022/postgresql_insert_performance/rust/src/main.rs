mod db;

use chrono::{DateTime, Utc};
use db::DB;
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::time::{Duration, Instant};
use uuid::Uuid;

const CONCURRENCY: u32 = 100;
const EXECUTIONS: u64 = 100_000;
const RUNS: usize = 10;

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

    let db = db::connect(&database_url, CONCURRENCY).await?;

    db::setup(&db).await?;

    // normalized
    println!("Normalized");
    let mut normalized_results = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        db::clean_table(&db, "normalized").await;
        let start = Instant::now();
        insert_normalized(&db).await;
        let duration = start.elapsed();
        normalized_results.push(duration);
    }
    println!("    results: {:#?}", &normalized_results);
    let normalized_mean = duration_mean(&normalized_results);
    println!("    mean: {:?}", &normalized_mean);

    // key_value
    println!("Key Value");
    let mut key_value_results = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        db::clean_table(&db, "key_value").await;
        let start = Instant::now();
        insert_key_value(&db).await;
        let duration = start.elapsed();
        key_value_results.push(duration);
    }
    println!("    results: {:#?}", &key_value_results);
    let key_value_mean = duration_mean(&key_value_results);
    println!("    mean: {:?}", &key_value_mean);

    Ok(())
}

fn duration_mean(results: &[Duration]) -> Duration {
    let result = results.iter().fold(Duration::ZERO, |acc, x| acc + *x);
    result / results.len() as u32
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

async fn insert_key_value_compressed_zstd(db: &DB) {
    const QUERY: &str = "INSERT INTO key_value_compressed_zstd (key, value)
        VALUES ($1, $2)";
    let stream = stream::iter(0..EXECUTIONS);
    let base_event = generate_event();
    let key_value_event = KeyValueEvent {
        key: base_event.id,
        value: serde_json::to_vec(&base_event)
            .expect("key_value_compressed_zstd: serializing event"),
    };

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| {
            let mut event = key_value_event.clone();
            event.key = Uuid::new_v4();
            async move {
                event.value = tokio::task::spawn_blocking(move || {
                    zstd::bulk::compress(&event.value, 2)
                        .expect("key_value_compressed_zstd :compressing event")
                })
                .await
                .expect("key_value_compressed_zstd: awaiting for tokio::spawn_blocking");

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

async fn insert_key_value_compressed_snappy(db: &DB) {
    const QUERY: &str = "INSERT INTO key_value_compressed_snappy (key, value)
        VALUES ($1, $2)";
    let stream = stream::iter(0..EXECUTIONS);
    let base_event = generate_event();
    let key_value_event = KeyValueEvent {
        key: base_event.id,
        value: serde_json::to_vec(&base_event)
            .expect("key_value_compressed_snappy: serializing event"),
    };

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| {
            let mut event = key_value_event.clone();
            event.key = Uuid::new_v4();
            async move {
                event.value = tokio::task::spawn_blocking(move || {
                    snap::raw::Encoder::new()
                        .compress_vec(&event.value)
                        .expect("key_value_compressed_snappy :compressing event")
                })
                .await
                .expect("key_value_compressed_snappy: awaiting for tokio::spawn_blocking");

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
    let json_event = serde_json::to_vec(&base_event).expect("timeseries: serializing event");

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| async {
            let timestamp = Utc::now();
            sqlx::query(QUERY)
                .bind(&timestamp)
                .bind(&json_event)
                .execute(db)
                .await
                .expect("timeseries: inserting event");
        })
        .await;
}
