pub fn main() {
  let hoge = 10;
  let vector = vec![10, 20, 30];
  let tuple = ([1, 2], vec![1, 2]);
  let ref_entire = &vector;
  let ref_elem = &vector[1];

  let ref_elem;
  {
    let vector = vec![10, 20, 30];
    {
      let ref_entire = &vector;
    }
    ref_elem = &vector[1];
  }

  let vector = vec![10, 20, 30];
  {
    let moved = vector;
  }

  let value = 10;
  let copied = value;
  assert_eq!(value, copied);

  let digits = {
    let mut tmp = Vec::new();
    for i in 0..10 {
      tmp.push(i);
    }
    tmp
  };
  println!("{:?}", digits);

  let digits = sequence(10);
  println!("{:?}", digits);

  let test_scores = vec![82, 91, 70];
  let total_score = sum(test_scores);

  assert_eq!(total_score, 252);

  let tuple: (Vec<i32>, i32) = (vec![], 7);
  let moved = tuple;

  let vector = vec![];
  let tuple: (Vec<i32>, i32) = (vector, 10);

  let tuple = (vec![10, 20, 30], 7);
  let vector = tuple.0;
  assert_eq!(tuple.1, 7);

  let vector: Vec<Vec<i32>> = vec![vec![2, 3, 4], vec![1], vec![0; 5]];
  for i in &vector {
    for j in i {
      print!("{} ", j);
    }
    println!();
  }
  println!("{:?}", vector);

  let vector = vec![1, 2, 3];
  let reference = &vector;
  let moved = vector;

  let tuple = (vec![0], 10);
  let (v, x) = tuple;
  assert_eq!(tuple.1, 10);

  let reference;
  let vector = {
    let v = vec![10, 20, 30];
    reference = &v;
    v
  };

  let reference;
  let vector = {
    let v = vec![10, 20, 30];
    reference = &v;
  };

  let vector = vec![
    (vec![5, 2], 3.5),
    (vec![1, 4, 3, 3], 2.75),
    (vec![4, 6], 5.),
  ];
  for &(ref v, mean) in &vector {
    println!("{:?}: {}", v, mean);
  }
}

fn sequence(n: usize) -> Vec<usize> {
  let mut ret = Vec::new();
  for i in 0..n {
    ret.push(i);
  }
  ret
}

fn sum(vector: Vec<i32>) -> i32 {
  let mut ret = 0;
  for elem in &vector {
    ret += elem;
  }
  ret
}
