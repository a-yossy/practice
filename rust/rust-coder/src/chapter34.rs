pub fn main() {
  let vec = vec![2, 4, 7, 8, 6, 3, 5];
  let result = increasing(&vec);
  assert_eq!(result, &[2, 4, 7, 8]);

  let vec = vec![2, 4, 7, 8, 6, 3, 5];
  let result;
  {
    let slice = &vec[..];
    result = increasing(slice);
  }
  assert_eq!(result, &[2, 4, 7, 8]);

  let four = vec![1, 2, 3, 4];
  let result;
  {
    let three = vec![1, 2, 3];
    result = longer(&four, &three);
    assert_eq!(&four, result);
  }

  let (former, latter) = split_mid(&[3, 1, 4, 1, 5, 2]);
  assert_eq!(former, &[3, 1, 4]);
  assert_eq!(latter, &[1, 5, 2]);
}

fn increasing<'a>(slice: &'a [i32]) -> &'a [i32] {
  let mut ret: &'a [i32] = slice;
  for i in 0..slice.len() - 1 {
    if slice[i] >= slice[i + 1] {
      ret = &slice[..=i];
      break;
    }
  }

  ret
}

fn add<'a>(x: &'a mut [i32], y: &[i32]) -> &'a [i32] {
  let len = x.len().min(y.len());
  for i in 0..len {
    x[i] += y[i]
  }

  &x[0..len]
}

fn longer<'a>(x: &'a [i32], y: &'a [i32]) -> &'a [i32] {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn split_mid(slice: &[i32]) -> (&[i32], &[i32]) {
  let mid = slice.len() / 2;
  (&slice[..mid], &slice[mid..])
}

fn ordinal_suffix(number: u32) -> &'static str {
  match (number % 10, number % 100) {
    (_, 11..=13) => "th",
    (1, _) => "st",
    (2, _) => "nd",
    (3, _) => "rd",
    _ => "th",
  }
}
