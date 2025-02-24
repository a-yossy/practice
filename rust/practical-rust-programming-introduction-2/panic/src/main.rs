use std::panic::{catch_unwind, resume_unwind, set_hook, take_hook};
use std::thread;

fn print_range(x: &[i32]) {
    let min = x.iter().min().unwrap();
    let max = x.iter().max().unwrap();
    eprintln!("max - min = {}", max - min);
}

struct A(bool);

impl Drop for A {
    fn drop(&mut self) {
        if self.0 {
            eprintln!("Something weird happened! cleaning... by Drop");
        }
    }
}

fn main() {
    let hook_orig = take_hook();
    set_hook(Box::new(move |info| {
        eprintln!("panic occured");
        hook_orig(info);
    }));
    let requests = vec![
        vec![1, 2, 3],
        vec![],
        vec![2000000000, -2000000000],
        vec![0, 42],
    ];
    for request in &requests {
        let result = catch_unwind(|| {
            print_range(request);
        });
        if let Err(_payload) = result {
            eprintln!("print_range failed");
        } else {
            eprintln!("success");
        }
    }

    let mut flag = A(true);
    assert!(false);
    flag.0 = false;

    let thread1 = thread::spawn(|| assert!(false));
    eprintln!("is_ok = {}", thread1.join().is_ok());

    let thread2 = thread::spawn(|| assert!(true));
    eprintln!("is_ok = {}", thread2.join().is_ok());

    let result = catch_unwind(|| {
        assert!(false);
    });

    if let Err(payload) = result {
        println!("Something weird happened! cleaning...");
        resume_unwind(payload);
    }

    eprintln!("Success!");
}
