async fn add(left: i32, right: i32) -> i32 {
  left + right
}

#[async_std::main]
pub async fn main() {
  let ans = add(2, 3).await;
  println!("{}", ans);
}