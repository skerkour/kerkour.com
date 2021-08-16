use std::time::Duration;
use libs::tokio::time;

pub async fn sleep_one() {
    time::sleep(Duration::from_secs(1)).await;
}
