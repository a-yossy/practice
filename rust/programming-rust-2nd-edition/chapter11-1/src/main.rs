use std::{
    fmt::{Debug, Display},
    fs::File,
    hash::Hash,
    io::Write,
    ops::{Add, Mul},
};

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_hello_2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }

    sum
}

trait Vegetable {}

struct Salad<V: Vegetable> {
    veggies: Vec<V>,
}

struct Salad2 {
    veggies: Vec<Box<dyn Vegetable>>,
}

trait Visible {
    fn draw(&self, canvas: &mut String);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> (i32, i32);
}

struct Broom;

trait StringSet {
    fn new() -> Self;
    fn from_slice(strings: &[&str]) -> Self;
    fn contains(&self, string: &str) -> bool;
    fn add(&mut self, string: &str);
}

fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }

    unknowns
}

trait StringSet2 {
    fn new() -> Self
    where
        Self: Sized;

    fn from_slice(strings: &[&str]) -> Self
    where
        Self: Sized;
}

fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }

    results
}

fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}:{:?}", index, value);
    }
}

fn dump2(iter: &mut dyn Iterator<Item = String>) {
    for (index, s) in iter.enumerate() {
        println!("{}:{:?}", index, s);
    }
}

trait Pattern {
    type Match;
    fn search(&self, string: &str) -> Option<Self::Match>;
}

impl Pattern for char {
    type Match = usize;
    fn search(&self, string: &str) -> Option<Self::Match> {
        Some(1)
    }
}

// pub trait Mul<RHS = Self> {
//     type Output;
//     fn mul(self, rhs: RHS) -> Self::Output;
// }

fn cycical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

trait Shape {
    fn new() -> Self;
    fn area(&self) -> f64;
}

struct Circle;
impl Shape for Circle {
    fn new() -> Self {
        Circle
    }

    fn area(&self) -> f64 {
        3.14
    }
}

struct Triangle;
impl Shape for Triangle {
    fn new() -> Self {
        Triangle
    }

    fn area(&self) -> f64 {
        0.5
    }
}

struct Rectangle;
impl Shape for Rectangle {
    fn new() -> Self {
        Rectangle
    }

    fn area(&self) -> f64 {
        1.0
    }
}

// fn make_shape(shape: &str) -> impl Shape {
//     match shape {
//         "circle" => Circle::new(),
//         "triangle" => Triangle::new(),
//         "rectangle" => Rectangle::new(),
//     }
// }

fn print<T: Display>(val: T) {
    println!("{}", val);
}

fn print2(val: impl Display) {
    println!("{}", val);
}

trait Greet {
    const GREETING: &'static str = "Hello";
    fn greet(&self) -> String;
}

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Float for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
    value + T::ONE
}

fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }

    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

fn main() -> std::io::Result<()> {
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;

    let w: Box<dyn Write> = Box::new(local_file);

    dot_product::<3>([0.2, 0.4, 0.6], [0., 0., 1.]);
    dot_product([3., 4.], [-5., 1.]);

    let mut sink = std::io::sink();
    say_hello(&mut sink)?;

    "hello".to_string();
    str::to_string("hello");
    ToString::to_string("hello");
    <str as ToString>::to_string("hello");

    let zero = 0;
    i64::abs(zero);

    let line = String::new();
    let words: Vec<_> = line.split_whitespace().map(ToString::to_string).collect();

    Ok(())
}
