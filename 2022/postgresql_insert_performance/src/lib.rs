use chrono::{DateTime, Utc};
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::Json, Executor, Pool, Postgres};
use std::time::Duration;
use uuid::Uuid;

type DB = Pool<Postgres>;
const CONCURRENCY: u32 = 100;

#[derive(sqlx::FromRow, Serialize, Debug, Clone, Deserialize)]
struct Event {
    id: uuid::Uuid,
    r#type: String,
    timestamp: DateTime<Utc>,
    received_at: DateTime<Utc>,
    payload: Json<Payload>,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
struct Payload {
    something: String,
    something_else: Vec<u8>,
    something_else2: Vec<u32>,
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
    CREATE TABLE IF NOT EXISTS normalized (
        id UUID PRIMARY KEY,
        type TEXT NOT NULL,
        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
        received_at TIMESTAMP WITH TIME ZONE NOT NULL,
        payload JSONB NOT NULL
    );
    ",
    )
    .await?;
    Ok(())
}

async fn insert_normalized(db: &DB) {
    const QUERY: &str = "INSERT INTO normalized (id, timestamp, received_at, payload)
        VALUES ($1, $2, $3, $4)";
    let stream = stream::iter(0..100_000);
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
                    .expect("inserting normalized event");
            }
        })
        .await;
}
