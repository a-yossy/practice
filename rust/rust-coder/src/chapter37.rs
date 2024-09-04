struct Vector(f64, f64);

impl Vector {
  fn length(&self) -> f64 {
    let Vector(x, y) = *self;
    (x * x + y * y).sqrt()
  }
}

impl Vector {
  fn into_tuple(self) -> (f64, f64) {
    (self.0, self.1)
  }
}

impl Vector {
  fn inverse(&mut self) {
    self.0 = -self.0;
    self.1 = -self.1;
  }
}

impl Vector {
  fn scale(&self, factor: f64) -> Vector {
    let Vector(x, y) = *self;
    Vector(factor * x, factor * y)
  }
}

impl Vector {
  fn zero() -> Vector {
    Vector(0., 0.)
  }
}

enum Shape {
  Triangle(f64, f64, f64),
  Rectangle { height: f64, width: f64 },
  Circle { radius: f64 }
}

impl Shape {
  fn area(&self) -> f64 {
    match *self {
      Shape::Triangle(a, b, c) => {
        let s = (a + b + c) / 2.;
        let squared = s * (s - a) * (s - b) * (s - c);
        squared.sqrt()
      }
      Shape::Rectangle {
        height: h,
        width: w
      } => h * w,
      Shape::Circle { radius } => radius * radius * std::f64::consts::PI,
    }
  }
}

pub fn main() {
  let v = Vector(1., 1.);
  let r = v.length();
  println!("{}", r);

  let v = Vector(1., 2.);
  let tuple = v.into_tuple();
  println!("{}", tuple.0);
  println!("{}", tuple.1);

  let mut v = Vector(2., 3.);
  v.inverse();
  println!("{}", v.0);
  println!("{}", v.1);

  let v = Vector(2., 3.);
  let scaled = v.scale(5.);
  println!("{}", scaled.0);
  println!("{}", scaled.1);
}
