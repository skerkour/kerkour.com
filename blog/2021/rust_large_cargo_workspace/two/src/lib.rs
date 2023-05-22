use libs::tokio::time;
use std::time::Duration;

pub async fn sleep_two() {
    time::sleep(Duration::from_secs(2)).await;
}
