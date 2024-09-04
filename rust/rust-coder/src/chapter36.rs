enum Shape {
  Triangle(f64, f64, f64),
  Rectangle { height: f64, width: f64 },
  Circle { radius: f64 },
}

pub fn main() {
}

fn area(shape: &Shape) -> f64 {
  match *shape {
    Shape::Triangle(a, b, c) => {
      let s = (a + b + c) / 2.;
      let squared = s * (s - a) * (s - b) * (s - c);
      squared.sqrt()
    }
    Shape::Rectangle {
      height: h,
      width: w,
    } => h * w,
    Shape::Circle { radius } => radius * radius * std::f64::consts::PI,
  }
}
