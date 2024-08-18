fn factorial(n: usize) -> usize {
  if n == 0 {
    1
  } else {
    n * factorial(n - 1)
  }
}

fn main() {
  factorial(10);
}
