use async_trait::async_trait;

#[async_trait]
trait AsyncTrait {
    async fn f() {
        println!("Could compile");
    }
}

#[async_trait]
pub trait AsyncTrait2 {
    async fn f(&self);
}

struct Runner {}

#[async_trait]
impl AsyncTrait2 for Runner {
    async fn f(&self) {
        println!("Hello, async-trait");
    }
}

async fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[async_std::main]
async  fn main() {
    let ans = add(2, 3).await;
    println!("{}", ans);
}
