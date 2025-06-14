use std::os::raw::{c_int, c_void};

#[no_mangle]
pub static RS_DEBUG: bool = true;

extern "C" {
    static FOREIGN_DEBUG: bool;
}

#[no_mangle]
pub extern "C" fn hello_rust(i: i32) {}

extern "C" {
    fn hello_foreign(i: i32);
}

#[repr(C)]
enum Foo {
    Bar = 1,
    Baz = 2,
}

#[repr(C)]
enum Foo2 {
    Bar(i32),
    Baz { a: bool, bo: f64 },
}

extern "C" fn ECDSA_SIG_new() -> *mut ECDSA_SIG;
extern "C" fn ECDSA_SIG_free(sig: *mut ECDSA_SIG);

extern "C" fn BIO_new_mem_buf(buf: *const c_void, len: c_int) -> *mut BIO;

extern "C" fn start_main_loop();
extern "C" fn next_event() -> *mut Event;

pub struct EventLoop(std::marker::PhantomData<*const ()>);
pub fn start() -> EventLoop {
    unsafe { ffi::start_main_loop() };
    EventLoop(std::marker::PhantomData)
}
impl EventLoop {
    pub fn next_event(&self) -> Option<Event> {
        let e = unsafe { ffi::next_event() };
    }
}

#[non_exhaustive]
#[repr(transparent)]
pub struct Foo(c_void);
#[non_exhaustive]
#[repr(transparent)]
pub struct Bar(c_void);
extern "C" {
    pub fn foo() -> *mut Foo;
    pub fn take_foo(arg: *mut Foo);
    pub fn take_bar(arg: *mut Bar);
}

fn main() {
    println!("Hello, world!");
}
