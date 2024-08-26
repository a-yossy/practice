fn fact(n: i32) -> i32 {
  if n == 0 {
    1
  } else {
    fact(n - 1) * n
  }
}

fn digit_sum(n: i32) -> i32 {
  if n == 0 {
    return 0;
  }

  let last_digit = n % 10;
  let higher_digits_sum = digit_sum(n / 10);
  let result = higher_digits_sum + last_digit;
  println!("{} の格桁の和は {} + {} = {}", n, higher_digits_sum, last_digit, result);
  result
}

fn gcd(m: i32, n: i32) -> i32 {
  if n == 0 {
    m
  } else {
    gcd(n, m % n)
  }
}

pub fn main() {
  let result = digit_sum(6318);
  println!("{}", result);
  println!("{}", gcd(18, 30));
  println!("{}", gcd(15, 24));
}
