use futures::future;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let now = Instant::now();

    let future_one = one_nyan();
    let future_two = two_nyan();
    let future_three = three_nyan();

    future::join3(future_one, future_two, future_three).await;

    println!("{}秒経過", now.elapsed().as_secs_f64());
}

async fn one_nyan() {
    println!("one_nyan() 開始");
    sleep(Duration::from_secs_f64(1.0)).await;
    println!("にゃー");
}

async fn two_nyan() {
    println!("two_nyan() 開始");
    sleep(Duration::from_secs_f64(0.5)).await;
    println!("にゃーにゃー");
}

async fn three_nyan() {
    println!("three_nyan() 開始");
    sleep(Duration::from_secs_f64(0.25)).await;
    println!("にゃーにゃーにゃー");
}
