use std::{
    cell::{Cell, RefCell},
    f64::consts::FRAC_PI_2,
    fs::File,
    io::Write,
    rc::Rc,
};

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    const NAME: &'static str = "Vector2";
    const ID: u32 = 18;
}

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
}

impl Queue<f64> {
    fn sum(&self) -> f64 {
        1.0
    }
}

struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }

    Extrema { greatest, least }
}

struct Polynomial<const N: usize> {
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Self {
        Self { coefficients }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}

struct LumpOfReferences<'a, T, const N: usize> {
    the_lump: [&'a T; N],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>,
}

impl SpiderRobot {
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }

    pub fn log(&self, message: &str) {
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message).unwrap();
    }
}

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>,
}

fn main() {
    let scaled = Vector2::UNIT;
    let mut q = Queue::<char>::new();

    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);

    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.).abs() < 0.005);

    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    }
    let mut w = ref_cell.borrow_mut();
    w.push_str(" world");
}
