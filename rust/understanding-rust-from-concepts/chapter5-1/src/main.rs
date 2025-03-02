fn return_input<T>(x: T) -> T {
    x
}

struct GenEx<T> {
    value: T
}

impl<T> GenEx<T> {
    fn return_value(self) -> T {
        self.value
    }
}

trait CalcLength {
    fn calc_length(&self) -> f64;
}

trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Line {
    length: f64
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTriangle {
    width: f64,
    height: f64,
}

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.width + self.height + (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height * 0.5
    }
}

fn length<T: CalcLength>(x: &T) -> f64 {
    x.calc_length()
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}

fn myadd<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let rect = Rectangle{ width: 1.0, height: 2.0 };
    println!("{}", area(&rect));
}
