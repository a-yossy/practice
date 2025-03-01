use serde_json::json;
use crate::RecursiveEnum::{Val, Null};

#[derive(Debug)]
enum RecursiveEnum {
    Val(Box<RecursiveEnum>),
    Null
}

const BUFSIZE: usize = 1024;
static OFFSET: usize = 15;

fn add_static() {
    const INCREMENT: usize = 2;
    static mut ADD: usize = 1;

    unsafe {
        ADD += INCREMENT;
        println!("add = {}", ADD);
    }
}

fn main() {
    let hanako = json!({
        "name": "hanako",
        "age": 8,
        "favorites": {
            "food": ["apple", "melon"]
        }
    });
    println!("hanako debug: {:?}", hanako);
    println!("hanako[\"name\"]: {}", hanako["name"]);
    println!("hanako[\"name\"]: {}", hanako["name"].as_str().unwrap());

    let mut taro = json!({});
    taro["name"] = json!("taro");
    taro["age"] = json!(10);
    taro["favorites"] = json!({"food": ["cake"]});

    let mut members = json!({});
    members["members"] = json!([taro, hanako]);

    println!("JSON: {}", members.to_string());

    let x = 1;
    let x_ptr: *const i32 = &x;

    println!("x_ptr = {}", unsafe{*x_ptr});

    let mut y = 20;
    let y_ptr = &mut y as *mut i32;

    unsafe {
        *y_ptr = *x_ptr + 1;
    }
    println!("y_ptr = {}", unsafe{*y_ptr});
    println!("y = {}", y);

    let boxed = Box::new(1);
    let val = *boxed;

    println!("*val = {}", val);

    let x = Val(Box::new(Val(Box::new(Null))));
    println!("{:?}", x);

    let offset_ref = &OFFSET;

    println!("bufsize = {}", BUFSIZE);
    println!("offset = {}", offset_ref);

    add_static();
    add_static();
    add_static();
}
