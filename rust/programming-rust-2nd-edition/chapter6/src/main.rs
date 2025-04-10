struct Process;
impl Process {
    fn wait(&self) -> bool {
        true
    }

    fn exit_code(&self) -> i32 {
        1
    }
}

fn serve_forever() -> ! {
    loop {
        println!("loop");
    }
}

fn main() {
    println!("Hello, world!");
    let is_even = |x| x % 2 == 0;
    let is_even2 = |x: u64| -> bool { x % 2 == 0 };

    let temp = is_even(11);
}
