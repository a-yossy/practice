use std::{
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
    thread::spawn,
};

fn main() {
    // static X: AtomicBool = AtomicBool::new(false);
    // static Y: AtomicBool = AtomicBool::new(false);

    let t1 = spawn(|| {
        let r1 = Y.load(Ordering::Relaxed); // 1
        X.store(r1, Ordering::Relaxed); // 2
    });
    let t2 = spawn(|| {
        let r2 = X.load(Ordering::Relaxed); // 3
        Y.store(true, Ordering::Relaxed); // 4
    });

    let t1 = spawn(|| {
        let r1 = Y.load(Ordering::Acquire);
        X.store(r1, Ordering::Release);
    });
    let t2 = spawn(|| {
        let r2 = X.load(Ordering::Acquire); // 1
        Y.store(true, Ordering::Release); // 2
    });

    static X: AtomicBool = AtomicBool::new(false);
    static Y: AtomicBool = AtomicBool::new(false);
    static Z: AtomicI32 = AtomicI32::new(0);

    let t1 = spawn(|| {
        X.store(true, Ordering::Release);
    });
    let t2 = spawn(|| {
        Y.store(true, Ordering::Release);
    });
    let t3 = spawn(|| {
        while (!X.load(Ordering::Acquire)) {}
        if (Y.load(Ordering::Acquire)) { // 1
            Z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t4 = spawn(|| {
        while (!Y.load(Ordering::Acquire)) {}
        if (X.load(Ordering::Acquire)) { // 2
            Z.fetch_add(1, Ordering::Relaxed);
        }
    });
}
