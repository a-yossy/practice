use std::result;

use proconio::input;

pub fn main() {
  for i in 0..=100 {
    if is_prime(i) {
      println!("{}", i);
    }
  }

  let result = fnc1_false() & fnc2_true();
  println!("{}", result);

  let result = fnc1_false() && fnc2_true();
  println!("{}", result);

  input! {
    x: i32
  }
  if (x != 0) && (12 % x == 0) {
    println!("{} は 12 の約数です", x);
  }

  let result = fnc1_true() | fnc2_false();
  println!("{}", result);

  let result = fnc1_true() || fnc2_false();
  println!("{}", result);

  let mut flag = true;
  flag &= false;
  println!("{}", flag);
  flag |= false;
  println!("{}", flag);
  flag |= true;
  println!("{}", flag);
  flag ^= true;
  println!("{}", flag);
}

fn is_prime(x: i32) -> bool {
  if x < 2 {
    return false;
  }

  for i in 2.. {
    if i * i > x {
      return true;
    }
    if x % i == 0 {
      return false;
    }
  }

  unreachable!();
}

fn fnc1_false() -> bool {
  println!("fnc1: false");
  false
}

fn fnc2_true() -> bool {
  println!("fnc2: true");
  true
}

fn fnc1_true() -> bool {
  println!("fnc1: true");
  true
}

fn fnc2_false() -> bool {
  println!("fnc2: false");
  false
}
