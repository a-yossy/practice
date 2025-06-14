use std::{
    borrow::Cow,
    fmt::Write,
    sync::atomic::{AtomicBool, Ordering},
};

struct EntityIdentifier<'a> {
    namespace: Cow<'a, str>,
    name: Cow<'a, str>,
}

fn mutex(lock: &AtomicBool, f: impl FnOnce()) {
    struct DropGuard<'a>(&'a AtomicBool);
    impl Drop for DropGuard<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::Release);
        }
    }
    let _guard = DropGuard(lock);
    f();
}

fn main() {
    let mut s = String::new();
    let x = 5;
    write!(&mut s, "{}+1={}", x, x + 1);
}
