use sqlx::{postgres::PgPoolOptions, Executor, Pool, Postgres};
use std::time::Duration;

pub type DB = Pool<Postgres>;

pub async fn connect(database_url: &str, max_connections: u32) -> Result<DB, anyhow::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await?;
    Ok(pool)
}

pub async fn setup(db: &DB) -> Result<(), anyhow::Error> {
    db.execute(
        "
    CREATE EXTENSION IF NOT EXISTS timescaledb;

    CREATE TABLE IF NOT EXISTS normalized (
        id UUID PRIMARY KEY,
        type TEXT NOT NULL,
        timestamp TIMESTAMPTZ NOT NULL,
        received_at TIMESTAMPTZ NOT NULL,
        payload JSONB NOT NULL
    );

    CREATE TABLE IF NOT EXISTS key_value (
        key UUID PRIMARY KEY,
        value BYTEA NOT NULL
    );

    CREATE TABLE IF NOT EXISTS key_value_compressed_zstd (
        key UUID PRIMARY KEY,
        value BYTEA NOT NULL
    );

    CREATE TABLE IF NOT EXISTS key_value_compressed_snappy (
        key UUID PRIMARY KEY,
        value BYTEA NOT NULL
    );

    CREATE TABLE IF NOT EXISTS timeseries (
        timestamp TIMESTAMPTZ NOT NULL,
        value BYTEA NOT NULL
    );
    CREATE INDEX index_timeseries_on_timestamp ON timeseries (timestamp);


    CREATE TABLE IF NOT EXISTS timeseries_timescale (
        timestamp TIMESTAMPTZ NOT NULL,
        value BYTEA NOT NULL
    );
    SELECT create_hypertable('timeseries_timescale', 'timestamp', if_not_exists => TRUE);

    ",
    )
    .await?;
    Ok(())
}

pub async fn clean_table(db: &DB, table: &str) {
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
