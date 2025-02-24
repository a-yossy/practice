use std::ptr::{read, write};
use std::mem::size_of_val;
use std::slice::from_raw_parts;
use std::slice;

pub fn replace_with<T, F>(r: &mut T, f: F)
where
    F: FnOnce(T) -> T
{
    let value = f(unsafe { read(r) });
    unsafe { write(r, value) };
}

pub fn anything_as_bytes<T: ?Sized>(val: &T) -> &[u8] {
    unsafe {
        from_raw_parts(val as *const T as *const u8,size_of_val(val))
    }
}

pub fn concat_slice<'a, T>(slice1: &'a [T], slice2: &'a [T]) -> Option<&'a [T]> {
    if unsafe { slice1.as_ptr().offset(slice1.len() as isize) } == slice2.as_ptr() {
        Some(unsafe { slice::from_raw_parts(slice1.as_ptr(), slice1.len() + slice2.len()) })
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
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

fn main() {
    let mut s = String::from("Hello");
    replace_with(&mut s, |s| s + ", world!");
}
