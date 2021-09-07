mod db;
mod error;
mod postgres;
mod queue;
pub use error::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::BadConfig("DATABASE_URL env var is missing".to_string()))?;

    let db = db::connect(&database_url).await?;
    db::migrate(&db).await?;


    Ok(())
}


