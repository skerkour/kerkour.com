mod db;
mod error;
mod postgres;
mod queue;
use std::{sync::Arc, time::Duration};

pub use error::Error;
use futures::{stream, StreamExt};
use postgres::PostgresQueue;
use queue::{Job, Message, Queue};

const CONCURRENCY: usize = 50;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::BadConfig("DATABASE_URL env var is missing".to_string()))?;

    let db = db::connect(&database_url).await?;
    db::migrate(&db).await?;

    let queue = Arc::new(PostgresQueue::new(db.clone()));

    // run worker
    let worker_queue = queue.clone(); // queue is an Arc pointer, so we only copy the reference
    tokio::spawn(async move { run_worker(worker_queue).await });

    // queue job
    let job = Message::SendSignInEmail {
        email: "your@email.com".to_string(),
        name: "Sylvain Kerkour".to_string(),
        code: "000-000".to_string(),
    };
    let _ = queue.push(job, None).await; // TODO: handle error

    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}

async fn run_worker(queue: Arc<dyn Queue>) {
    loop {
        let jobs = match queue.pull(CONCURRENCY as u32).await {
            Ok(jobs) => jobs,
            Err(err) => {
                println!("run_worker: pulling jobs: {}", err);
                tokio::time::sleep(Duration::from_millis(500)).await;
                Vec::new()
            }
        };

        let number_of_jobs = jobs.len();
        if number_of_jobs > 0 {
            println!("Fetched {} jobs", number_of_jobs);
        }

        stream::iter(jobs)
            .for_each_concurrent(CONCURRENCY, |job| async {
                let job_id = job.id;

                let res = match handle_job(job).await {
                    Ok(_) => queue.delete_job(job_id).await,
                    Err(err) => {
                        println!("run_worker: handling job({}): {}", job_id, &err);
                        queue.fail_job(job_id).await
                    }
                };

                match res {
                    Ok(_) => {}
                    Err(err) => {
                        println!("run_worker: deleting / failing job: {}", &err);
                    }
                }
            })
            .await;

        // sleep not to overload our database
        tokio::time::sleep(Duration::from_millis(125)).await;
    }
}

async fn handle_job(job: Job) -> Result<(), crate::Error> {
    match job.message {
        message @ Message::SendSignInEmail { .. } => {
            println!("Sending sign in email: {:?}", &message);
        }
    };

    Ok(())
}
