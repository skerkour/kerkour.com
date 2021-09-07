# How to build a job queue with Rust and PostgreSQL


```shell
$ docker run --name rust_job_queue -d -e POSTGRES_USER=rust -e POSTGRES_PASSWORD=job_queue -p 5432:5432 postgres:latest
$ DATABASE_URL=postgres://rust:job_queue@localhost:5432/rust cargo run
```
