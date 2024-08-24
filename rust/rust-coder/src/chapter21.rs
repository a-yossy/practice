fn fact5() -> i32 {
  let mut n = 1;
  for i in 1..=5 {
    print!("{} ", n);
    n *= i;
  }
  println!("{}", n);
  n
}


fn prod(a: i32, b: i32) -> i32 {
  let mut n = 1;
  for i in a..=b {
    n *= i
  }
  n
}

fn swap((a, b): (i32, i32)) -> (i32, i32) {
  (b, a)
}

fn double(mut x: i32) -> i32 {
  x *= x;
  x
}

fn minimum_factor(n: i32) -> i32 {
  for i in 2.. {
    if i * i > n {
      break;
    } else if n % i == 0 {
      return i;
    }
  }
  n
}

pub fn main() {
  let value = fact5();

  {
    fn greet() {
      println!("Hello");
    }
    greet();
  }
  let tuple = (5, 10);
  swap(tuple);

  let a = 10;
  let b = 20;
  fn fnc() {
      let c = 40;
      println!("{}", c);
  }

  let var = 5;
  assert_eq!(double(var), 25);
  assert_eq!(var, 5);

  assert_eq!(minimum_factor(2021), 43);
}
