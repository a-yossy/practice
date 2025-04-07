use std::clone;

struct File {
    descriptor: i32
}

fn new_file(d: i32) -> File {
    File { descriptor: d }
}

fn close_from(this: &mut File, rhs: &File) {
    
}

fn main() {
    println!("Hello, world!");
}
