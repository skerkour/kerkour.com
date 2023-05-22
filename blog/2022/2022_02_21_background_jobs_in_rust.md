+++
date = 2022-02-21T02:00:00Z
title = "Background jobs in Rust"
type = "post"
tags = ["rust", "programming", "tutorial", "webdev"]
authors = ["Sylvain Kerkour"]
url = "/rust-background-jobs"

[extra]
lang = "en"

comment ="""

"""
+++


## Recurrent jobs

For recurrent jobs (a.k.a. CRON jobs), I personnaly use the [lightspeed_scheduler](https://crates.io/crates/lightspeed_scheduler) crate.


```rust
  let job = Job::new("kernel", "dispatch_delete_old_data", Some(3), move || {
      let kernel_service_inner = kernel_service.clone();
      Box::pin(async move {
          kernel_service_inner.dispatch_delete_old_data().await?;
          Ok(())
      })
  });
  scheduler
      .add_job_with_scheduler(
          "* 0 4 * * *"
              .to_scheduler()
              .expect("scheduler.run: parsing kernel.delete_old_data cron expression"),
          job,
      )
      .await;

  info!("scheduler.run: Starting scheduler.");
  scheduler.run().await?.await?;
```

A good (and maybe simpler) alternative is [tokio-cron-scheduler](https://crates.io/crates/tokio-cron-scheduler).

```rust
  let mut scheduler = JobScheduler::new();

  scheduler.add(Job::new("1/10 * * * * *", |uuid, l| {
      println!("I run every 10 seconds");
  }).expect("defining 10 seeconds job"));

  scheduler.add(Job::new_async("1/7 * * * * *", |uuid, l| Box::pin( async {
      println!("I run async every 7 seconds");
  })).expect("defining 7 seeconds job"));

  scheduler.start().await;
```

## One-off background jobs

For one-off background jobs, such as dispatching transactional emails, we need a queue.

I already covered this topic in [how to build a job queue with Rust and PostgreSQL](/rust-job-queue-with-postgresql), and I still strongly recommend this solution for both its operational simplicity and performance!

```rust
  // queue job
  let job = Message::SendSignInEmail {
      email: "your@email.com".to_string(),
      name: "Sylvain Kerkour".to_string(),
      code: "000-000".to_string(),
  };
  let _ = queue.push(job, Some(Utc::now())).await?;
```
