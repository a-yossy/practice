use proconio::input;

pub fn main() {
  let ref_slice: &[i32] = &[10, 15];
  if let [x, y, z] = *ref_slice {
    println!("{} {} {}", x, y, z);
  } else {
    println!("失敗");
  }


  let vector = [(1, 1), (2, 2), (3, 3), (0, 0)];
  for &tuple in &vector {
    if let (1, value) = tuple {
      println!("{}", value)
    } else if let (2, value) = tuple {
      println!("{}", value * value)
    } else if let (0, 0) = tuple {
      break;
    } else {
      println!("?");
    }
  }

  let array = [(1, 92), (3, 91), (95, 1), (94, 2)];
  let mut vector = Vec::new();
  for &tuple in &array {
    if let (1, value) | (value, 2) = tuple {
      vector.push(value);
    }
  }
  println!("{:?}", vector);

  let tuple = (3, 1, 2);
  if let (1, _, _) | (_, 1, _) | (_, _, 1) = tuple {
    println!("1がある");
  }

  let x = 10;
  let _ = x;

  for _ in 0..4 {
    println!("test");
  }

  let array = [0, 0, 0, 1, 2];
  let mut ref_slice = &array[..];
  while let [0, ..] = *ref_slice {
    ref_slice = &ref_slice[1..];
  }
  println!("{:?}", ref_slice);

  let vector = [(1, 1), (2, 2), (3, 3), (0, 0)];
  for &tuple in &vector {
    match tuple {
        (1, value) => println!("{}", value),
        (2, value) => println!("{}", value * value),
        (0, 0) => break,
        _ => println!("?"),
    }
  }

  let tuple = (1, 1);
  match tuple {
      (1, 1) => println!("1"),
      (1, _) | (_, 1) => println!("片方が1"),
      _ => println!("1 ではない")
  }

  loop {
      input! {
        x: i32,
      }
      match x {
        0 => {
          break;
        }
        n => {
          for _ in 0..n {
            print!("!")
          }
          println!();
        }
      }
  }

  input! {
    x: i32,
  }
  let y = match x {
      0 => 1,
      1 => 0,
      _ => {
        println!("neither 0 nor 1");
        0
      }
  };
  println!("{}", y);

  input! {
    x: i32,
  }
  match x % 3 {
    0 => println!("3"),
    1 | -2 => println!("-1"),
    2 | -1 => println!("+1"),
    _ => unreachable!()
  }

  loop {
      input! {
        x: i32,
      }
      let y = match x {
          0 => break,
          x => x - 1,
      };
      println!("{}", y);
  }

  let tuple = (1, 3);
  match tuple {
      (1, x) if x % 2 == 0 => println!("{}", x),
      _ => {}
  }

  let tuples = [(2, 5), (4, 4), (1, -4), (-3, -3)];
  let mut vector = Vec::new();
  for &tuple in &tuples {
    match tuple {
      (x, y) if x == y => vector.push(x),
      _ => {}
    }
  }
  println!("{:?}", vector);
}
