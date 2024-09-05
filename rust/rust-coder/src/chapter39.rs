pub fn main() {
  let x = Some(10);
  let y = None;
  let z: i32 = match y {
    Some(i) => i,
    None => -1,
  };

  assert!(Some(10) == Some(10));
  assert!(Some(10) != Some(15));
  assert!(Some(10) != None);
  assert!(Option::<i32>::None == None);

  assert!(Some(10) < Some(15));
  assert!(Some(10) > None);

  let x = Some(10);
  let copied = x;
  assert_eq!(x, Some(10));

  let h = 10;
  let w = 10;
  let coin;
  let obstacle;
  let mut table: Vec<Vec<Option<u64>>> = vec![vec![None; w]; h];
  table[0][0] = Some(coin[0][0]);
  for i in 0..h {
    for j in 0.. w {
      if obstacle[i][j] {
        continue;
      }
      let above = if i > 0 { table[i - 1][j] } else { None };
      let left = if j > 0 { table[i][j - 1] } else { None };
      if let Some(c) = above.max(left) {
        table[i][j] = Some(c + coin[i][j]);
      }
    }
  }

  match table[h - 1][w - 1] {
    Some(ans) => println!("{}", ans),
    None => println!("Impossible"),
  }

  let mut v= vec![10];
  assert_eq!(v.pop(), Some(10));
  assert_eq!(v.pop(), None);
  let a = [3, 1, 4, 1, 5];
  assert_eq!(a.get(2), Some(&4));
  assert_eq!(a.get(5), None);

  let x: i32 = 1000000000;
  let y = 2000000000;
  assert!(x.checked_add(y).is_none());
}
