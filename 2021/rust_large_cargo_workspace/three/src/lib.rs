use std::time::Duration;
use libs::tokio::time;

pub async fn sleep_three() {
    time::sleep(Duration::from_secs(3)).await;
}
