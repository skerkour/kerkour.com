mod db;

use chrono::{DateTime, Utc};
use db::DB;
use futures::{stream, StreamExt};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::time::{Duration, Instant};
use uuid::Uuid;

const CONCURRENCY: u32 = 100;
const EXECUTIONS: u64 = 25_000;
const RUNS: usize = 6;

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
    let normalized_inserts_per_sec = EXECUTIONS as f64 / normalized_mean.as_secs_f64();
    println!(
        "    mean: {:?} ({:.2} inserts / s)",
        &normalized_mean, normalized_inserts_per_sec
    );

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
    let key_value_inserts_per_sec = EXECUTIONS as f64 / key_value_mean.as_secs_f64();
    println!(
        "    mean: {:?} ({:.2} inserts / s)",
        &key_value_mean, key_value_inserts_per_sec
    );

    // key_value zstd
    println!("Key Value compressed ZSTD");
    let mut key_value_zstd_results = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        db::clean_table(&db, "key_value_compressed_zstd").await;
        let start = Instant::now();
        insert_key_value_compressed_zstd(&db).await;
        let duration = start.elapsed();
        key_value_zstd_results.push(duration);
    }
    println!("    results: {:#?}", &key_value_zstd_results);
    let key_value_zstd_mean = duration_mean(&key_value_zstd_results);
    let key_value_zstd_inserts_per_sec = EXECUTIONS as f64 / key_value_zstd_mean.as_secs_f64();
    println!(
        "    mean: {:?} ({:.2} inserts / s)",
        &key_value_zstd_mean, key_value_zstd_inserts_per_sec
    );

    // key_value snappy
    println!("Key Value compressed snappy");
    let mut key_value_snappy_results = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        db::clean_table(&db, "key_value_compressed_snappy").await;
        let start = Instant::now();
        insert_key_value_compressed_snappy(&db).await;
        let duration = start.elapsed();
        key_value_snappy_results.push(duration);
    }
    println!("    results: {:#?}", &key_value_snappy_results);
    let key_value_snappy_mean = duration_mean(&key_value_snappy_results);
    let key_value_snappy_inserts_per_sec = EXECUTIONS as f64 / key_value_snappy_mean.as_secs_f64();
    println!(
        "    mean: {:?} ({:.2} inserts / s)",
        &key_value_snappy_mean, key_value_snappy_inserts_per_sec
    );

    // timeseries
    println!("Timeseries");
    let mut timeseries_results = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        db::clean_table(&db, "timeseries").await;
        let start = Instant::now();
        insert_timeseries(&db).await;
        let duration = start.elapsed();
        timeseries_results.push(duration);
    }
    println!("    results: {:#?}", &timeseries_results);
    let timeseries_mean = duration_mean(&timeseries_results);
    let timeseries_inserts_per_sec = EXECUTIONS as f64 / timeseries_mean.as_secs_f64();
    println!(
        "    mean: {:?} ({:.2} inserts / s)",
        &timeseries_mean, timeseries_inserts_per_sec
    );

    // timeseries timescale
    println!("Timeseries timescale");
    let mut timeseries_timescale_results = Vec::with_capacity(RUNS);
    for _ in 0..RUNS {
        db::clean_table(&db, "timeseries_timescale").await;
        let start = Instant::now();
        insert_timeseries_timescale(&db).await;
        let duration = start.elapsed();
        timeseries_timescale_results.push(duration);
    }
    println!("    results: {:#?}", &timeseries_timescale_results);
    let timeseries_timescale_mean = duration_mean(&timeseries_timescale_results);
    let timeseries_timescale_inserts_per_sec =
        EXECUTIONS as f64 / timeseries_timescale_mean.as_secs_f64();
    println!(
        "    mean: {:?} ({:.2} inserts / s)",
        &timeseries_timescale_mean, timeseries_timescale_inserts_per_sec
    );

    Ok(())
}

fn duration_mean(results: &[Duration]) -> Duration {
    let result = results.iter().fold(Duration::ZERO, |acc, x| acc + *x);
    result / results.len() as u32
}

fn generate_event() -> Event {
    let now = Utc::now();
    let mut rng = rand::thread_rng();
    let something_else = (0..500).into_iter().map(|_| rng.gen()).collect();
    let something_else2 = (0..500).into_iter().map(|_| rng.gen()).collect();
    let something = rng
        .sample_iter(&Alphanumeric)
        .take(500)
        .map(char::from)
        .collect();

    let payload = Payload {
        something,
        something_else,
        something_else2,
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

async fn insert_timeseries_timescale(db: &DB) {
    const QUERY: &str = "INSERT INTO timeseries_timescale (timestamp, value)
        VALUES ($1, $2)";
    let stream = stream::iter(0..EXECUTIONS);
    let base_event = generate_event();
    let json_event =
        serde_json::to_vec(&base_event).expect("timeseries_timescale: serializing event");

    stream
        .for_each_concurrent(CONCURRENCY as usize, |_| async {
            let timestamp = Utc::now();
            let json_event = json_event.clone();
            let data = tokio::task::spawn_blocking(move || {
                zstd::bulk::compress(&json_event, 2)
                    .expect("key_value_compressed_zstd :compressing event")
            })
            .await
            .expect("timeseries_timescale: awaiting for tokio::spawn_blocking");
            sqlx::query(QUERY)
                .bind(&timestamp)
                .bind(&data)
                .execute(db)
                .await
                .expect("timeseries_timescale: inserting event");
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
            let json_event = json_event.clone();
            let data = tokio::task::spawn_blocking(move || {
                zstd::bulk::compress(&json_event, 2).expect("timeseries :compressing event")
            })
            .await
            .expect("timeseries: awaiting for tokio::spawn_blocking");
            sqlx::query(QUERY)
                .bind(&timestamp)
                .bind(&data)
                .execute(db)
                .await
                .expect("timeseries: inserting event");
        })
        .await;
}
