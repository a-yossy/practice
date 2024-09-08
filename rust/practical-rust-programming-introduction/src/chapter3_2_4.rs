use async_trait::async_trait;

#[async_trait]
trait AsyncTrait {
  async fn f() {
    println!("Could not compile")
  }
}

#[async_trait]
trait AsyncTrait2 {
  async fn f(&self);
}

struct Runner {}

#[async_trait]
impl AsyncTrait2 for Runner {
  async fn f(&self) {
      println!("Hello, async-trait");
  }
}

pub fn main() {

}
