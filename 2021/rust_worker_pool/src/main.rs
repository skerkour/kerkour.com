////////////////////////////////////////////////////////////////////////////////////////////////////
// Sync
////////////////////////////////////////////////////////////////////////////////////////////////////
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;

fn compute_job(job: i64) -> i64 {
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(0..10);
    std::thread::sleep(Duration::from_millis(sleep_ms));

    job * job
}

fn process_result(result: i64) {
    println!("{}", result);
}

fn main() {
    let jobs = 0..100;

    jobs.into_par_iter()
        .map(compute_job)
        .for_each(process_result);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Async
////////////////////////////////////////////////////////////////////////////////////////////////////
// use futures::{stream, StreamExt};
// use rand::{thread_rng, Rng};
// use std::time::Duration;

// async fn compute_job(job: i64) -> i64 {
//     let mut rng = thread_rng();
//     let sleep_ms: u64 = rng.gen_range(0..10);
//     tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

//     job * job
// }

// async fn process_result(result: i64) {
//     println!("{}", result);
// }

// #[tokio::main(flavor = "multi_thread")]
// async fn main() {
//     let jobs = 0..100;
//     let concurrency = 42;

//     stream::iter(jobs)
//         .for_each_concurrent(concurrency, |job| async move {
//             let result = compute_job(job).await;
//             process_result(result).await;
//         })
//         .await;

//    stream::iter(jobs)
//     .map(compute_job)
//     .buffer_unordered(concurrency)
//     .for_each(process_result)
//     .await;

//    let results: Vec<i64> = stream::iter(jobs)
//     .map(compute_job)
//     .buffer_unordered(concurrency)
//     .collect()
//     .await;
// }
