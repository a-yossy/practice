pub fn main() {
  let _vec = Vec::<i32>::new();
  let vector: Vec<i32> = vec![1, 2, 3];
  println!("{}", vector.len());
  println!("{} {} {}", vector[0], vector[1], vector[2]);
  let mut mut_vector = vec![1, 2, 3];
  println!("{:?}", mut_vector);
  mut_vector[1] = 10;
  println!("{:?}", mut_vector);

  let vector: Vec<u32>;
  vector = Vec::new();

  let mut vector = Vec::new();
  println!("{}", vector.len());

  vector.push(10);
  vector.push(20);
  vector.push(30);
  println!("{:?}", vector);

  vector.push(40_i64);
  vector.push(50);
  println!("{:?}", vector);

  vector.pop();
  println!("{:?}", vector);

  vector.pop();
  vector.pop();
  vector.pop();
  println!("{:?}", vector);

  proconio::input! {
    n: usize,
    vector: [i32; n],
  }

  println!("{}", n);
  println!("{:?}", vector);

  let vector = vec![30, 20, 30];
  let mut sum = 0;
  for num in &vector {
    sum += num;
  }
  println!("{}", sum);
}
