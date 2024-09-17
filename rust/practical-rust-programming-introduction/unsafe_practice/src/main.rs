use core::slice;
use std::{mem::MaybeUninit, ptr::{read, write}, slice::from_raw_parts, usize};

fn sum(a: &[i32]) -> i32 {
    let mut ptr = a.as_ptr();
    let end = unsafe { ptr.offset(a.len() as isize) };
    let mut sum = 0;
    while ptr < end {
        sum += unsafe { *ptr };
        ptr = unsafe { ptr.offset(1) };
    }
    sum
}

pub fn foo(ptr: *const i32) {
    if ptr.is_null() {
        eprintln!("ptr is null");
    } else {
        println!("ptr is not null");
    }
    eprintln!("*ptr is {}", unsafe { *ptr });
}

pub fn replace_with<T, F>(r: &mut T, f: F)
where
    F: FnOnce(T) -> T
{
    let value = f(unsafe { read(r) });
    unsafe { write(r, value) };
}

pub fn anything_as_bytes<T: ?Sized>(val: &T) -> &[u8] {
    unsafe {
        from_raw_parts(val as *const T as *const u8, size_of_val(val))
    }
}

pub fn concat_slice<'a, T>(slice1: &'a [T], slice2: &'a [T]) -> Option<&'a [T]> {
    if unsafe { slice1.as_ptr().offset(slice1.len() as isize) } == slice2.as_ptr() {
        Some(unsafe { slice::from_raw_parts(slice1.as_ptr(), slice1.len() + slice2.len()) })
    } else {
        None
    }
}


pub fn lower_bound(slice: &[i32], target: i32) -> usize {
    let (mut lo, mut hi) = (0, slice.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if slice[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32
}

impl Rect {
    pub fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        if right <= left || bottom <= top {
            Rect {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0
            }
        } else {
            Rect { left, right, top, bottom }
        }
    }
}

// #[repr(transparent)]
// pub struct str([u8]);
pub fn first_char(s: &str) -> Option<char> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let first_byte = bytes[0];
    if first_byte < 0x80 {
        Some(first_byte as char)
    } else if first_byte < 0xDF {
        let secound_byte = unsafe { *bytes.get_unchecked(1) };
        todo!()
    } else {
        todo!()
    }
}

pub fn split_at_dot(s: &str) -> (&str, &str) {
    if let Some(dot) = s.find(".") {
        unsafe {
            (s.get_unchecked(..dot), s.get_unchecked(dot + 1..))
        }
    } else {
        (s, "")
    }
}

pub unsafe fn from_utf8_unchecked(bytes: &[u8]) -> &str {
    todo!()
}
// impl<T: ?Sized> Box<T> {
//     pub unsafe fn from_raw(raw: *mut T) -> Box<T> {
//         todo!()
//     }
// }
// pub trait SliceIndex<T: ?Sized>: Sealed {
//     unsafe fn get_unchecked(self, slice: &T) -> &Self::Output {
//         todo!()
//     }
// }
pub unsafe trait TrustedLen: Iterator {}
// pub unsafe auto trait Send {}
// pub fn register_something(hook: unsafe extern "C" fn(*mut c_void)) {
//     todo!()
// }

pub unsafe fn from_begin_end<'a>(begin: *const u8, end: *const u8) -> &'a [u8] {
    std::slice::from_raw_parts(begin, end as usize - begin as usize);

    unsafe {
        std::slice::from_raw_parts(begin, end as usize - begin as usize)
    }
}

pub fn repeat_str(s: &str, n: usize) -> String {
    let len = s.len() * n;
    let mut ret = vec![0; len];
    for i in 0..n {
        for j in 0..s.len() {
            unsafe {
                *ret.get_unchecked_mut(i * s.len() + j) = *s.as_bytes().get_unchecked(j);
            }
        }
    }
    unsafe { String::from_utf8_unchecked(ret) }
}

#[test]
fn test_repeat_str() {
    repeat_str("abcdefgh", usize::MAX / 8 + 1);
}

fn main() {
    let mut s = String::from("Hello");
    replace_with(&mut s, |s| s + ", world!");

    // let mut s = String::from("Hello");
    // replace_with(&mut s, |_| panic!());

    eprintln!("{:x?}", anything_as_bytes(&42));
    eprintln!("{:x?}", anything_as_bytes(&42.0));
    eprintln!("{:x?}", anything_as_bytes(&(42, 42.0)));
    let cell = std::cell::Cell::new(42);
    let bytes = anything_as_bytes(&cell);
    cell.set(84);
    eprintln!("bytes = {:x?}", bytes);

    let x: &i32 = unsafe { MaybeUninit::zeroed().assume_init() };
}
