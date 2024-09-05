pub fn main() {
  let mut x = 5;
  println!("{}", x);
  x = 6;
  println!("{}", x);

  let source = vec![1, 2, 3, 4, 5];
  let result = source
    .into_iter()
    .filter(|n| n % 2 == 0)
    .map(|n| n.to_string())
    .collect::<Vec<String>>();

  let objective: Option<i32> = Some(1);
  match objective {
    Some(x) if x % 2 == 0 => println!("偶数 {}", x),
    Some(x) => println!("奇数, {}", x),
    None => println!("値なし")
  }

  let mut v = vec![];
  v.push(1);
}
