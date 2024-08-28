pub fn main() {
  let mut ref_slice: &[i32];

  let array = [1, 2, 3];
  ref_slice = &array;
  println!("{:?}", ref_slice);
  println!("{:p}", ref_slice);

  let vector = vec![4, 5, 6];
  ref_slice = &vector;
  println!("{:?}", ref_slice);

  let ref_slice: &[i32] = &[50, 20, 30];
  println!("{}", ref_slice[0]);
  println!("{}", ref_slice[1]);
  println!("{}", ref_slice[2]);

  for &i in ref_slice {
    println!("{}", i);
  }

  let array = [0, 10, 20, 30, 40, 50];
  let ref_slice = &array[..];
  println!("{:?}", ref_slice);

  let empty = &array[1..1];
  println!("{:?}", empty);

  let mut array = [0, 10, 20, 30, 40];
  let ref_mut_slice = &mut array[..];
  println!("{:?}", ref_mut_slice);
  ref_mut_slice.swap(1, 3);
  println!("{:?}", ref_mut_slice);
  println!("{:?}", array);

  let mut array = [7, 2, -3, 9, -2, 5];
  array.reverse();
  println!("{:?}", array);

  let mut array = [7, 2, -3, 9, -2, 5];
  array[1..4].reverse();
  println!("{:?}", array);

  let mut array = [7, 2, 5, -3, 9, -2, 5];
  array.sort();
  println!("{:?}", array);
}
