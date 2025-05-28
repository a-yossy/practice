use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
};

#[repr(C)]
pub struct git_error {
    pub message: *const c_char,
    pub klass: c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum git_error_code {
    GIT_OK = 0,
    GIT_ERROR = -1,
    GIT_ENOTFOUND = -3,
    GIT_EEXISTS = -4,
}

#[repr(C)]
enum Tag {
    Float = 0,
    Int = 1,
}

#[repr(C)]
union FloatOrInt {
    f: f32,
    i: i32,
}

#[repr(C)]
struct Value {
    tag: Tag,
    union: FloatOrInt,
}

fn is_zero(v: Value) -> bool {
    use self::Tag::*;
    unsafe {
        match v {
            Value {
                tag: Int,
                union: FloatOrInt { i: 0 },
            } => true,
            Value {
                tag: Float,
                union: FloatOrInt { f: num },
            } => num == 0.0,
            _ => false,
        }
    }
}

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    static environ: *mut *mut c_char;
}

#[link(name = "git2")]
extern "C" {
    pub fn git_libgit2_init() -> c_int;
    pub fn git_libgit2_shutdown() -> c_int;
}

fn main() {
    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();
    unsafe {
        assert_eq!(strlen(null_terminated.as_ptr()), 12);
    }

    unsafe {
        if !environ.is_null() && !(*environ).is_null() {
            let var = CStr::from_ptr(*environ);
            println!("first environment variable: {}", var.to_string_lossy());
        }
    }

    unsafe {
        git_libgit2_init();
        git_libgit2_shutdown();
    }
}
