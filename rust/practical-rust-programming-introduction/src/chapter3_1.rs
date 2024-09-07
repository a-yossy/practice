use std::io::Write;

pub fn main() {
  let s1: String = String::from("Hello, World!");
  let s2: &str = &s1;
  let s3: String = s2.to_string();

  let mut t = (1, "2");
  t.0 = 2;
  t.1 = "3";

  let mut a: [i32; 3] = [0, 1, 2];
  let b: [i32; 3] = [0; 3];
  a[1] = b[1];
  a[2] = b[2];
  let new_a = &a[1..3];
  println!("{:?}", &a[1..3]);

  struct Person {
    name: String,
    age: u32
  }
  let p = Person {
    name: String::from("John"),
    age: 8
  };

  enum Event {
    Quit,
    KeyDown(u8),
    MouseDown { x: i32, y: i32 }
  }
  let e1 = Event::Quit;
  let e2 = Event::MouseDown { x: 10, y: 10 };

  let result: Result<i32, String> = Ok(200);
  match result {
    Ok(code) => println!("code: {}", code),
    Err(err) => println!("Err: {}", err),
  }

  let result: Result<i32, String> = Ok(200);
  if let Ok(code) = result {
    println!("code: {}", code);
  }

  let result: Result<i32, String> = Ok(200);
  println!("code: {}", result.unwrap_or(-1));
  let result: Result<i32, String> = Err("error".to_string());
  println!("code: {}", result.unwrap_or(-1));

  fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
  }
  let result: Result<i32, String> = Ok(200);
  let next_result = result.and_then(func);
  let result: Result<i32, String> = Err("error".to_string());
  let next_result = result.and_then(func);

  fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?;
    println!("code: {}", code);
    Ok(100)
  }

  let v1 = vec![1, 2, 3, 4, 5];
  let v2 = vec![0; 5];
  let v = vec![1, 2, 3, 4, 5];
  println!("{}", v[0]);

  let v = vec![1, 2, 3, 4, 5];
  for element in &v {
    println!("{}", element);
  }

  fn print(s: Box<[u8]>) {
    println!("{:?}", s);
  }
  let byte_array = [b'h', b'e', b'l', b'l', b'o'];
  print(Box::new(byte_array));
  let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
  print(Box::new(byte_array));

  let immut_val = 10;
  let mut mut_val = 20;
  mut_val += immut_val;
  let v1: u64 = 10;
  let v2 = 10u64;

  let number = 1;
  if 0 < number {
    println!("0 < number");
  } else if number < 0 {
    println!("number < 0");
  } else {
    println!("0 == number");
  }
  let number = 1;
  let result = if 0 <= number {
    number
  } else {
    -number
  };

  let mut count = 0;
  let result = loop {
    println!("count: {}", count);
    count += 1;
    if count == 10 {
      break count;
    }
  };

  let mut count = 0;
  while count < 10 {
    println!("count :{}", count);
    count += 1;
  }

  let count: i32;
  for count in 0..10 {
    println!("count: {}", count);
  }

  let array = [0, 1, 2, 3];
  for element in &array {
    println!("element: {}", element);
  }

  'main: loop {
    println!("main loop start");
    'sub: loop {
      println!("sub loop start");

      break 'main;

      println!("sub loop end");
    }
    println!("main loop end");
  }

  let i: i32 = 1;
  match i {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("misc")
  }

  enum Color {
    Red,
    Blue,
    Green,
  }
  let c = Color::Red;
  match c {
    Color::Red => println!("Red"),
    Color::Blue => println!("Blue"),
    Color::Green => println!("Green")
  }

  let result: Result<i32, String> = Ok(100);
  let result_number = match result {
    Ok(number) => number,
    Err(message) => {
      println!("Error: {}", message);
      -1
    }
  };

  for number in 1..5 {
    println!("{}", number);
  }

  struct Iter {
    current: usize,
    max: usize
  }
  impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
      self.current += 1;
      if self.current - 1 < self.max {
        Some(self.current - 1)
      } else {
        None
      }
    }
  }
  let it = Iter {
    current: 0,
    max: 10,
  };
  for num in it {
    println!("{}", num);
  }

  fn add (a: i32, b: i32) -> i32 {
    a + b
  }
  let x = add(1, 2);
  println!("x = {}", x);

  fn abs(number: i32) -> i32 {
    if number < 0 {
      return -number;
    }
    number
  }

  struct Person2 {
    name: String,
    age: u32
  }
  impl Person2 {
    fn say_name(&self) -> &Self {
      println!("I am {}.", self.name);
      self
    }

    fn say_age(&self) -> &Self {
      println!("I am {} year(s) old.", self.age);
      self
    }

    fn new(name: &str, age: u32) -> Self {
      Person2 {
        name: String::from(name),
        age: age
      }
    }
  }
  let p = Person2{
    name: String::from("Taro"),
    age: 20
  };
  p.say_name().say_age();

  let p = Person2::new("Taro", 20);
  p.say_name().say_age();

  let s = concat!("A", "b2", 3);
  let s = format!("{}-{:?}", s, ("D", 5));
  let s = format!("{}-{}", "abc", "def");

  print!("hello");
  println!("hello {}", "world");
  eprint!("hello {}", "error");
  eprintln!("hello");

  let mut w = Vec::new();
  write!(&mut w, "{}", "ABC");
  writeln!(&mut w, " is 123");
  dbg!(w);

  // panic!("it will panic");

  let v = vec![1, 2, 3];

  println!("defined in file: {}", file!());
  println!("defined on line: {}", line!());
  println!("is test: {}", cfg!(unix));
  println!("CARGO_HOME: {}", env!("CARGO_HOME"));

  assert!(true);
  assert_eq!(1, 1);
  assert_ne!(1, 0);
  // debug_assert!(false);
  debug_assert_eq!(1, 1);
  debug_assert_ne!(1, 0);

  enum Emotion {
    Anger,
    Happy
  }
  trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
  }
  struct HappyPerson {
    name: String,
    state: Emotion
  }
  impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
      unimplemented!()
    }
    fn get_happy(&mut self) -> String {
      format!("{} is always happy.", self.name)
    }
    fn tell_state(&self) -> String {
      todo!()
    }
  }
  let mut p = HappyPerson {
    name: "Takashi".to_string(),
    state: Emotion::Happy,
  };
  println!("{}", p.get_happy());

  fn f(x: usize) -> &'static str {
    match x {
      n if n * n % 3 == 0 => "3n",
      n if n * n % 3 == 1 => "3n + 1 or 3n + 2",
      _ => unreachable!()
    }
  }

  #[derive(Eq, PartialEq)]
  struct A(i32);
  println!("{:?}", A(0) == A(1));

  #[derive(PartialOrd, PartialEq)]
  struct B(f32);
  println!("{:?}", B(1.0) > B(0.0));

  #[derive(Copy, Clone)]
  struct C;
  let c0 = C;
  let _c1 = c0;
  let _c2 = c0;

  #[derive(Clone)]
  struct D;
  let d0 = D;
  let _d1 = d0.clone();

  #[derive(Debug)]
  struct E;
  println!("{:?}", E);

  #[derive(Default)]
  struct F;
  let _f = F::default();

  let x = 0.0 / 0.0;
  println!("{:?}", x == x);

  let mut x = vec![0.1, 0.5, 0.3, 0.4, 0.2];
  x.sort_by(|a, b| a.partial_cmp(b).unwrap());
  println!("{:?}", x);

  let tmp = 0 / 0;
  println!("{}", tmp);
}
