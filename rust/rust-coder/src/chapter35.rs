struct Point(i32, i32);

pub fn main() {
  struct Vector(i32, i32);

  let point: Point;
  point = Point(2, 3);
  println!("{}", point.0);
  println!("{}", point.1);

  let mut point = Point(2, 3);
  std::mem::swap(&mut point.0, &mut point.1);
  println!("{}", point.0);
  println!("{}", point.1);

  let tuple: (i32,) = (5,);
  struct Length(i32);
  let length = Length(10);
  println!("{}", length.0);

  struct Physical {
    height: i32,
    weight: i32,
  }
  let david = Physical {
    height: 170,
    weight: 50
  };
  println!("{}", david.height);
  println!("{}", david.weight);

  let point = Point(1, 2);
  let Point(x, y) = point;
  println!("{} {}", x, y);

  let david = Physical {
    height: 170,
    weight: 50
  };
  let Physical {
    height: h,
    weight: w
  } = david;
  println!("{} {}", h, w);

  let Physical {
    height: _,
    weight: w
  } = david;
  let Physical { weight: w, .. } = david;

  let height = 170;
  let david = Physical {
    height,
    weight: 50,
  };
  let Physical {
    height,
    weight: w
  } = david;
}
