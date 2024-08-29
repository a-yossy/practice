pub fn main() {
  let vector = vec![20, 80, 60, 40];
  let s = sum(&vector);
  println!("{}", s);

  println!("{:?}", vector);

  let mut hoge = 10;
  double(&mut hoge);
  println!("{}", hoge);
  double(&mut hoge);
  println!("{}", hoge);

  let mut hoge = 10;
  let mut fuga = 20;
  fnc1(&hoge, &fuga);
  fnc2(&hoge, &mut fuga);
  fnc3(&mut hoge, &mut fuga);

  let mut x = 20;
  let mut y = 30;
  println!("x: {} y: {}", x, y);
  std::mem::swap(&mut x, &mut y);
  println!("x: {}, y: {}", x, y);

  proconio::input! {
    i: usize,
    j: usize
  }
  let mut array = [1, 2, 3, 4, 5];
  println!("{:?}", array);
  array.swap(i, j);
  println!("{:?}", array);

  let mut x = 0;
  for i in 18..=20 {
    x += i;
    dbg!(x);
  }
  println!("{}", x);
}

fn sum(v: &Vec<i32>) -> i32 {
  let mut ret = 0;
  for &i in v {
    ret += i;
  }
  ret
}

fn double(x: &mut i32) {
  *x *= 2;
}

fn fnc1(x: &i32, y: &i32) {}
fn fnc2(x: &i32, y: &mut i32) {}
fn fnc3(x: &mut i32, y: &mut i32) {}
