struct PointI32(i32, i32);
struct PointI64(i64, i64);
struct PointF64(f64, f64);
struct Point<T>(T, T);


impl<T> Point<T> {
  fn abscissa(&self) -> &T {
    &self.0
  }
}

pub fn main() {
  let p1: Point<i32> = Point::<i32>(1, 5);
  let p2: Point<i64> = Point::<i64>(1, 5);

  let p = Point::<i32>(1, 5);
  println!("{}", p.abscissa());
  let new_p = Point::<i64>(1, 5);
  println!("{}", p.abscissa());
}
