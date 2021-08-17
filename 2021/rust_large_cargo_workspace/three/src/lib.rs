use libs::tokio::time;
use std::time::Duration;

pub async fn sleep_three() {
    time::sleep(Duration::from_secs(3)).await;
}
