fn main() {
    let objective: Option<i32> = Some(1);
    match objective {
        Some(x) if x % 2 == 0 => println!("偶数 {}", x),
        Some(x) => println!("奇数 {}", x),
        None => println!("値なし")
    }
}
