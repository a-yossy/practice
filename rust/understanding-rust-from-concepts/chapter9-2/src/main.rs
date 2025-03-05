#[repr(C)]
struct Point {
    x: f64,
    y: f64
}

extern "C" {
    fn distance(a: Point, b: Point) -> f64;
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 4.0, y: 6.0 };

    unsafe {
        let dist = distance(a, b);
        println!("distance = {}", dist);
    }
}
