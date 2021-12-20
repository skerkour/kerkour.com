use libs::tokio;

#[tokio::main]
async fn main() {
    one::sleep_one().await;
    two::sleep_two().await;
    three::sleep_three().await;
}
