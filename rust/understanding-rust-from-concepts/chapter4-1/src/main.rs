use std::{fs::File, io::{BufRead, BufReader}};
use thiserror::Error;
use std::collections::HashMap;
use serde_json::json;

#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided by zero")]
    DivByZero(i32),
    #[error("Both numerator {0} and denominator {1} are negative")]
    BothNegative(i32, i32),
}

fn mydiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn repeat_mydiv(ary: &[(i32, i32)]) -> Result<Vec<i32>, DivError> {
    let mut ret = Vec::new();
    for aa in ary {
        let ans = mydiv(aa.0, aa.1)?;
        ret.push(ans);
        println!("pushed: {} / {} = {}", aa.0, aa.1, ans);
    }
    Ok(ret)
}

fn print_repeat_mydiv(result: Result<Vec<i32>, DivError>) {
    match result {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e)
    }
}

fn print_mydiv(x: i32, y: i32) {
    match mydiv(x, y) {
        Ok(ans) => println!("no error. ans = {}", ans),
        Err(e) => println!("{}", e)
    }
}

fn print_mydiv2(x: i32, y: i32) {
    let ret = mydiv(x, y);
    if ret.is_ok() {
        println!("no error. ans = {}", ret.unwrap());
    } else {
        println!("{}", ret.err().unwrap());
    }
}

fn print_num(x: i32) {
    println!("input: {}", x);
    let s = match x {
        0 => Some("zero"),
        1 | 2 => Some("small"),
        3..=9 => Some("large"),
        _ => None
    };
    match s {
        Some(st) => {
            println!("output: {}", st);
        },
        None => {
            println!("Not supported");
        }
    }
    println!();
}

enum OddEven {
    Odd,
    Even,
}

fn odd_or_even(x: i32) -> OddEven {
    if x % 2 == 0 {
        OddEven::Even
    } else {
        OddEven::Odd
    }
}

fn print_only_even(x: i32) {
    if let OddEven::Even = odd_or_even(x) {
        println!("{} is Even", x);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capitals = HashMap::new();
    capitals.insert("Japan", "Tokyo");
    capitals.insert("UK", "London");
    capitals.insert("US", "Washington D.C.");

    let targets = vec!["Japan", "US", "France"];
    for tg in targets {
        match capitals.get(tg) {
            Some(s) => println!("The capital of {} is {}", tg, s),
            None => println!("The capital of {} is not found", tg),
        }
    }

    print_mydiv(5, 2);
    print_mydiv(-5, 0);
    print_mydiv(-5, -2);

    println!("1st calc");
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (9, 3)]));
    println!();

    println!("2nd calc");
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-6, -3), (9, 3)]));
    println!();

    println!("3rd calc");
    print_repeat_mydiv(repeat_mydiv(&[(2, 1), (-6, 0), (6, 3)]));
    println!();

    let f = File::open("./input.txt")?;
    let f = BufReader::new(f);

    for line in f.lines().flatten() {
        let mut v = Vec::new();
        for ee in line.split(' ') {
            v.push(ee.parse()?);
        }
        let result = mydiv(v[0], v[1])?;
        println!("{} / {} = {}", v[0], v[1], result);
    }

    println!("{}", mydiv(3, 2)?);
    Ok(())
}
