use std::panic::take_hook;
use std::thread;
use std::{panic::catch_unwind, vec, panic::resume_unwind, panic::set_hook};

fn print_range(x: &[i32]) {
    let min = x.iter().min().unwrap();
    let max = x.iter().max().unwrap();
    eprintln!("max - min = {}", max - min);
}

fn main() {
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

    let thread1 = thread::spawn(|| assert!(false));
    eprintln!("is ok = {}", thread1.join().is_ok());
    let thread2 = thread::spawn(|| assert!(true));
    eprintln!("is_ok = {}", thread2.join().is_ok());

    let result = catch_unwind(|| {
        assert!(false);
    });
    if let Err(payload) = result {
        eprintln!("Something weird happened! cleaning...");
        resume_unwind(payload);
    }
    eprintln!("Success!");

    struct A(bool);
    impl Drop for A {
        fn drop(&mut self) {
            if self.0 {
                eprintln!("Something weird happened! cleaning...");
            }
        }
    }
    let mut flag = A(true);
    assert!(false);
    flag.0 = false;
    eprintln!("Success!");

    set_hook(Box::new(|_| eprintln!("panic occurred")));
    assert!(false);

    let hook_orig = take_hook();
    set_hook(Box::new(move |info| {
        eprintln!("panic occurred");
        hook_orig(info);
    }));
    assert!(false);

    struct Account {
        transactions: Vec<i32>,
        sum: i32
    }
    impl Account {
        fn add(&mut self, tx: i32) {
            self.transactions.push(tx);
            self.sum += tx;
        }
    }

    fn do_something(_state: &mut i32) {}
    let mut state = 0;
    catch_unwind(|| {
        do_something(&mut state);
    });
    
}
