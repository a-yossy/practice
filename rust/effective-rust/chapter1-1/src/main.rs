use std::collections::{HashMap, HashSet};

fn main() {
    #[derive(Debug, Clone, Copy)]
    enum HttpResultCode {
        Ok = 200,
        NotFound = 404,
        Teapot = 418,
    }
    let code = HttpResultCode::NotFound;
    assert_eq!(code as i32, 404);
    let msg = match code {
        HttpResultCode::Ok => "OK",
        HttpResultCode::NotFound => "Not Found",
        HttpResultCode::Teapot => "I'm a teapot",
    };

    pub enum Sides {
        Both,
        Single,
    }
    pub enum Output {
        BlackAndWhite,
        Color,
    }
    pub fn print_page(sides: Sides, color: Output) {}
    print_page(Sides::Both, Output::Color);

    pub type CpuId = u32;
    pub type Job = u32;
    pub enum SchedulerState {
        Inert,
        Pending(HashSet<Job>),
        Running(HashMap<CpuId, Vec<Job>>),
    }

    pub struct RgbColor(u8, u8, u8);
    pub struct DisplayProps {
        pub x: u32,
        pub y: u32,
        pub monochrome: bool,
        pub fg_color: RgbColor,
    }

    pub enum Color {
        Monochrome,
        Foreground(RgbColor),
    }
    pub struct DisplayProps2 {
        pub x: u32,
        pub y: u32,
        pub color: Color,
    }

    fn div(x: f64, y: f64) -> f64 {
        if y == 0.0 {
            return f64::NAN;
        }

        x / y
    }

    fn show(x: f64) {
        println!("x = {x}");
    }

    enum Shape {
        Rectangle { width: f64, height: f64 },
        Circle { radius: f64 },
    }
    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Rectangle { width, height } => width * height,
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            }
        }
    }

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    let op: fn(i32, i32) -> i32 = sum;
    let op1 = op;
    let op2 = op;
    assert!(op1 == op2);
    println!("op = {:p}", op);
    // let opt1 = sum;
    // let opt2 = sum;
    // assert!(opt1 == opt2);

    pub fn modify_all<F>(data: &mut [u32], mut mutator: F)
    where
        F: FnMut(u32) -> u32,
    {
        for value in data {
            *value = mutator(*value);
        }
    }

    fn add2(v: u32) -> u32 {
        v + 2
    }
    let mut data = [1, 2, 3];
    modify_all(&mut data, add2);
    assert_eq!(data, [3, 4, 5]);

    let amount_to_add = 3;
    struct InternalContext<'a> {
        amount_to_add: &'a u32,
    }
    impl<'a> InternalContext<'a> {
        fn internal_op(&self, y: u32) -> u32 {
            y + *self.amount_to_add
        }
    }
    let add_n = InternalContext {
        amount_to_add: &amount_to_add,
    };
    let z = add_n.internal_op(5);
    assert_eq!(z, 8);
    let add_n = |y| y + amount_to_add;

    let mut data = [1, 2, 3];
    modify_all(&mut data, add_n);
    assert_eq!(data, [4, 5, 6]);

    pub trait Sort {
        fn sort(&mut self);
    }
    pub trait StableSort: Sort {}

    pub fn dump_sorted<T>(mut collection: T)
    where
        T: Sort + IntoIterator,
        T::Item: std::fmt::Debug,
    {
        collection.sort();
        for item in collection {
            println!("{:?}", item);
        }
    }

    struct S {
        field: Option<i32>,
    }
    let s = S { field: Some(42) };
    // match &s.field {
    //     Some(i) => println!("filed is {i}"),
    //     None => {}
    // }
    if let Some(i) = &s.field {
        println!("field is {i}");
    }

    // let result = std::fs::File::open("/etc/passwd");
    // let f = match result {
    //     Ok(f) => f,
    //     Err(_e) => panic!("Failed to open file"),
    // };
    let f = std::fs::File::open("/etc/passwd").unwrap();

    // pub fn find_user(username: &str) -> Result<i64, std::io::Error> {
    //     let f = std::fs::File::open("/etc/passwd")?;
    //     Ok(10)
    // }

    // pub fn find_user(username: &str) -> Result<(), String> {
    //     let f = match std::fs::File::open("/etc/passwd") {
    //         Ok(file) => file,
    //         Err(e) => return Err(format!("Failed to open file: {}", e)),
    //     };

    //     Ok(())
    // }
    pub fn find_user(username: &str) -> Result<(), String> {
        let f = std::fs::File::open("/etc/passwd")
            .map_err(|e| format!("Failed to open file: {}", e))?;
        Ok(())
    }

    pub fn encrypt(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
    struct InputData {
        payload: Option<Vec<u8>>,
    }
    impl InputData {
        pub fn payload(&self) -> Vec<u8> {
            encrypt(self.payload.as_ref().unwrap_or(&vec![]))
        }
    }

    // pub type MyError = String;
    // impl std::error::Error for MyError {}

    #[derive(Debug)]
    pub struct MyError(String);
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl std::error::Error for MyError {}
    impl From<String> for MyError {
        fn from(value: String) -> Self {
            Self(value)
        }
    }
    pub fn find_user2(username: &str) -> Result<(), MyError> {
        let f = std::fs::File::open("/etc/passwd")
            .map_err(|e| format!("Failed to open file: {}", e))?;
        Ok(())
    }
}
