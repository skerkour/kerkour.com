+++
date = 2021-09-07T14:00:00Z
title = "How to build a job queue with Rust and PostgreSQL"
type = "post"
tags = ["rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/rust-job-queue-with-postgresql"

[extra]
lang = "en"

comment ="""
"""
+++

(or in any other language such as Go, Node.js or python)


Job queues are a central piece of any web application. It enables background jobs, buffering to handle surges in traffic, async messaging, batching and many other great things.


Thus, this is all natural that many vendors are offering various kinds of solutions.


But, all these solutions have a problem. **A big, awful problem**.

They come with a **high operational cost**: whether it be managing servers, storage, and security for on-premise solutions, or handling permissions and dealing with limitations (such as AWS SQS' delay, limited to 15 minutes) for managed solutions. Any new moving piece requires a lot of work not to go down or to avoid being hacked.

What if, instead of adding another chunk of complexity to our architecture, we could reuse something we already have. Something that benefits from thousands (millions?) of man-hours of work. Which specific purpose is to store data (after all, a job queue is simply a kind of specialized data store). Something that is **secure by default**, and, more importantly, something that **all cloud providers** are offering as a managed solution.

You get it! I'm talking about our old friend: **the Database** (here we are specifically talking about PostgreSQL, but it also applies to all other database, SQL or NoSQL).

With very little code, **PostgreSQL can be turned into a job queue** that can handle more than 100 jobs / second on a modest instance (which is approximately 260 000 000 jobs per months!).

If performance ever become a problem, you can dramatically increase throuput by batching `INSERT`s into transactions. If still not enough (we are talking about > 500 000 000 jobs a day), congratulation, you have found product-market fit. You should now be able to pay an entire team to optimize it, or move to a more performant solution.


## The interface

First, we define an interface, so we can easily switch to another backend at will.

In Rust, interfaces are defined with [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html).

```rust
#[async_trait::async_trait]
pub trait Queue: Send + Sync + Debug {
    async fn push(
        &self,
        job: Message,
        scheduled_for: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), crate::Error>;
    /// pull fetches at most `number_of_jobs` from the queue.
    async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, crate::Error>;
    async fn delete_job(&self, job_id: Uuid) -> Result<(), crate::Error>;
    async fn fail_job(&self, job_id: Uuid) -> Result<(), crate::Error>;
    async fn clear(&self) -> Result<(), crate::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    SendSignInEmail {
        email: String,
        name: String,
        code: String,
    },
    DeleteOldUserData,
    SendNewsletterMessage {
        message_id: Uuid,
    },
    // ...
}
```


## Implementing the job queue

We use [`sqlx`](https://github.com/launchbadge/sqlx) as our Postgres driver as it's, in my opinion, the best tradeoff between ease of use and efficiency.

There are **two special tricks** that make this implementation efficient, so **stay attentive** ;)

**Cargo.toml**
```toml
[package]
name = "rust_postgresql_job_queue"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.5", features = [ "runtime-tokio-rustls", "postgres", "chrono", "uuid", "json" ] }
chrono = "0.4"
uuid = { version = "0.8", features = ["serde", "v4"] }
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
thiserror = "1"
anyhow = "1"
ulid = { version = "0.4", features = ["uuid"] }
futures = "0.3"
```

First, our database model:

**migrations/0000_init.sql**
```sql
CREATE TABLE queue (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  scheduled_for TIMESTAMP WITH TIME ZONE NOT NULL,
  failed_attempts INT NOT NULL,
  status INT NOT NULL,
  message JSONB NOT NULL
);
CREATE INDEX index_queue_on_scheduled_for ON queue (scheduled_for);
CREATE INDEX index_queue_on_status ON queue (status);
```


Then the job as it will be stored in the database:
```rust
#[derive(sqlx::FromRow, Debug, Clone)]
struct PostgresJob {
    id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,

    scheduled_for: chrono::DateTime<chrono::Utc>,
    failed_attempts: i32,
    status: PostgresJobStatus,
    message: Json<Message>,
}

// We use a INT as Postgres representation for performance reasons
#[derive(Debug, Clone, sqlx::Type, PartialEq)]
#[repr(i32)]
enum PostgresJobStatus {
    Queued,
    Running,
    Failed,
}

impl From<PostgresJob> for Job {
    fn from(item: PostgresJob) -> Self {
        Job {
            id: item.id,
            message: item.message.0,
        }
    }
}
```

And finally, we can implement the actual queue:
```rust
#[derive(Debug, Clone)]
pub struct PostgresQueue {
    db: DB,
    max_attempts: u32,
}

impl PostgresQueue {
    pub fn new(db: DB) -> PostgresQueue {
        let queue = PostgresQueue {
            db,
            max_attempts: 5,
        };

        queue
    }
}
```

**Here is our first trick**: In order to reduce index fragmentation due to randomness (and thus improve performances) and allow job ordering, we don't use a `UUIDv4`. Instead, **we use a [ULID](https://github.com/ulid/spec)** that we convert to a `UUID` so there is no problem to serialize / deserialize it:
```rust
#[async_trait::async_trait]
impl Queue for PostgresQueue {
    async fn push(
        &self,
        job: Message,
        date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), crate::Error> {
        let scheduled_for = date.unwrap_or(chrono::Utc::now());
        let failed_attempts: i32 = 0;
        let message = Json(job);
        let status = PostgresJobStatus::Queued;
        let now = chrono::Utc::now();
        let job_id: Uuid = Ulid::new().into(); // ULID to UUID
        let query = "INSERT INTO queue
            (id, created_at, updated_at, scheduled_for, failed_attempts, status, message)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        sqlx::query(query)
            .bind(job_id)
            .bind(now)
            .bind(now)
            .bind(scheduled_for)
            .bind(failed_attempts)
            .bind(status)
            .bind(message)
            .execute(&self.db)
            .await?;
        Ok(())
    }
```

Then comes our **second trick: [`FOR UPDATE SKIP LOCKED`](https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE)**. Without that, our queue would perform very poorly!

```rust
    async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, crate::Error> {
        let now = chrono::Utc::now();
        let query = "UPDATE queue
            SET status = $1, updated_at = $2
            WHERE id IN (
                SELECT id
                FROM queue
                WHERE status = $3 AND scheduled_for <= $4 AND failed_attempts < $5
                ORDER BY scheduled_for
                FOR UPDATE SKIP LOCKED
                LIMIT $6
            )
            RETURNING *";

        let jobs: Vec<PostgresJob> = sqlx::query_as::<_, PostgresJob>(query)
            .bind(PostgresJobStatus::Running)
            .bind(now)
            .bind(PostgresJobStatus::Queued)
            .bind(now)
            .bind(MAX_FAILED_ATTEMPTS)
            .bind(number_of_jobs)
            .fetch_all(&self.db)
            .await?;
        Ok(jobs.into_iter().map(Into::into).collect())
    }
```

And finally, `delete_job`, `fail_job` and `clear`, which are all pretty straightforward to implement, no magic involved.
```rust
    async fn delete_job(&self, job_id: Uuid) -> Result<(), crate::Error> {
        let query = "DELETE FROM queue WHERE id = $1";

        sqlx::query(query).bind(job_id).execute(&self.db).await?;
        Ok(())
    }

    async fn fail_job(&self, job_id: Uuid) -> Result<(), crate::Error> {
        let now = chrono::Utc::now();
        let query = "UPDATE queue
            SET status = $1, updated_at = $2, failed_attempts = failed_attempts + 1
            WHERE id = $3";

        sqlx::query(query)
            .bind(PostgresJobStatus::Queued)
            .bind(now)
            .bind(job_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn clear(&self) -> Result<(), crate::Error> {
        let query = "DELETE FROM queue";

        sqlx::query(query).execute(&self.db).await?;
        Ok(())
    }
}
```


Now that everything is setup, here is how we can use this queue:
```rust
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
```

Note that we use a `Arc<Queue>` [smart pointer](https://kerkour.com/rust-avoid-lifetimes/) in order to have shared, long-lived references to the queue.




## The code is on GitHub

As usual, you can find the code on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/2021/rust_postgresql_job_queue) (please don't forget to star the repo 🙏).


It can be tested with docker:

```shell
$ docker run --name rust_job_queue -d -e POSTGRES_USER=rust -e POSTGRES_PASSWORD=job_queue -p 5432:5432 postgres:latest
$ DATABASE_URL=postgres://rust:job_queue@localhost:5432/rust cargo run
```
