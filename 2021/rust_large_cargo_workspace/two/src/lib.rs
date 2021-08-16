use std::time::Duration;
use libs::tokio::time;

pub async fn sleep_two() {
    time::sleep(Duration::from_secs(2)).await;
}
