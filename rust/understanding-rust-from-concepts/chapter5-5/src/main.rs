trait MyError: std::fmt::Debug{}

#[derive(Debug)]
struct MyError1;
impl MyError for MyError1 {}

#[derive(Debug)]
struct MyError2;
impl MyError for MyError2 {}

#[derive(Debug)]
struct MyError3;
impl MyError for MyError3 {}

#[derive(Debug)]
struct MyErrorOther;
impl MyError for MyErrorOther {}

fn div4(x: i32) -> Result<(), Box<dyn MyError>> {
    let res = x % 4;
    match res {
        0 => Ok(()),
        1 => Err(Box::new(MyError1 {})),
        2 => Err(Box::new(MyError2 {})),
        3 => Err(Box::new(MyError3 {})),
        _ => Err(Box::new(MyErrorOther {}))
    }
}

fn generic_return_type() -> Box<dyn std::fmt::Display> {
    Box::new(1)
}

fn generic_return_type2() -> impl std::fmt::Display {
    1
}

fn main() {
    println!("{:?}", div4(0));
    println!("{:?}", div4(1));
    println!("{:?}", div4(2));
    println!("{:?}", div4(3));

    let mut v = Vec::<Box<dyn std::fmt::Debug>>::new();
    v.push(Box::new(1_i32));
    v.push(Box::new(2_i32));
    v.push(Box::new(3.0_f64));
    v.push(Box::new(String::from("Hello")));
}
