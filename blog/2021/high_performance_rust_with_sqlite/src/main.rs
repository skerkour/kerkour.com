use clap::{App, Arg};
use futures::*;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};
use std::time::{Duration, Instant};
use std::{fs, str::FromStr};

struct User {
    id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_matches = App::new("Rust to the mooooon")
        .version("1.0")
        .arg(
            Arg::with_name("concurrency")
                .short("c")
                .long("concurrency")
                .help("Number of concurrent inserts")
                .default_value("3"),
        )
        .arg(
            Arg::with_name("inserts")
                .short("i")
                .long("inserts")
                .help("Number of inserts to perform")
                .default_value("40000"),
        )
        .get_matches();

    let concurrency = cli_matches
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1);
    let inserts = cli_matches
        .value_of("inserts")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1);

    let database_file = "db.sqlite";
    let database_url = format!("sqlite://{}", database_file);
    let pool_timeout = Duration::from_secs(30);
    // with pool_max_connections = 1, the pool timeout maybe related to https://github.com/launchbadge/sqlx/issues/1210
    let pool_max_connections = if concurrency == 1 {
        2
    } else {
        concurrency as u32
    };

    let _ = fs::remove_file(database_file);

    let connection_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(pool_timeout);

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(pool_max_connections)
        .connect_timeout(pool_timeout)
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!("./db").run(&sqlite_pool).await?;

    // sqlx::query("pragma journal_mode = WAL;")
    //     .execute(&sqlite_pool)
    //     .await?;
    // sqlx::query("pragma synchronous = normal;")
    //     .execute(&sqlite_pool)
    //     .await?;
    sqlx::query("pragma temp_store = memory;")
        .execute(&sqlite_pool)
        .await?;
    sqlx::query("pragma mmap_size = 30000000000;")
        .execute(&sqlite_pool)
        .await?;
    sqlx::query("pragma page_size = 4096;")
        .execute(&sqlite_pool)
        .await?;
    // sqlx::query("pragma busy_timeout = 50000;")
    //     .execute(&sqlite_pool)
    //     .await?;

    println!(
        "Inserting {} records. concurrency: {}",
        inserts, concurrency
    );

    let start = Instant::now();
    insert(inserts, concurrency, &sqlite_pool).await;
    let duration = start.elapsed();

    let inserts_per_sec = inserts as f64 / duration.as_secs_f64();
    println!(
        "Time elapsed to insert {} records: {:?} ({:.2} inserts/s)",
        inserts, duration, inserts_per_sec
    );

    Ok(())
}

async fn insert(inserts: usize, concurrency: usize, sqlite_pool: &Pool<Sqlite>) {
    let stream = stream::iter(0..inserts);

    stream
        .for_each_concurrent(concurrency, |_| async move {
            let user = User {
                id: uuid::Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                username: String::from("Hello"),
            };

            sqlx::query(
                "INSERT INTO users (id, created_at, username)
            VALUES (?, ?, ?)",
            )
            .bind(user.id)
            .bind(user.created_at)
            .bind(&user.username)
            .execute(sqlite_pool)
            .await
            .expect("inserting in db");
        })
        .await;
}
