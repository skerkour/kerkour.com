use libs::tokio::time;
use std::time::Duration;

pub async fn sleep_one() {
    time::sleep(Duration::from_secs(1)).await;
}
