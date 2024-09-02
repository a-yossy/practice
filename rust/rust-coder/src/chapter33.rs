pub fn main() {
  println!("{}", seconde_f64_i32((5., 3)));
  println!("{}", seconde_any_i32::<f64>((3_f64, 5)));
  println!("{}", seconde_any_i32::<f32>((3_f32, 5)));
  println!("{}", seconde_any_i32::<bool>((true, 5)));

  let result = second((true, 65));
  assert_eq!(result, b'A');

  print3(10);
  print3("Hello");
}

fn seconde_f64_i32(tuple: (f64, i32)) -> i32 {
  tuple.1
}

fn seconde_any_i32<T>(tuple: (T, i32)) -> i32 {
  tuple.1
}

fn second<T, U>(tuple: (T, U)) -> U {
  let _first: T = tuple.0;
  let second: U = tuple.1;
  second
}

fn print_i32(x: i32) {
  println!("{}", x);
}

fn print<T: std::fmt::Display>(x: T) {
  println!("{}", x);
}

fn print1(x: impl std::fmt::Display) {
  println!("{}", x);
}

fn print2<T>(x: T)
where
  T: std::fmt::Display,
{
  println!("{}", x);
}

fn print3<T>(x: T) -> T
where
  T: std::fmt::Display,
{
  println!("{}", x);
  x
}

fn print_display_and_debug<T: std::fmt::Display + std::fmt::Debug>(x: T) {
  println!("{}", x);
  println!("{:?}", x);
}
