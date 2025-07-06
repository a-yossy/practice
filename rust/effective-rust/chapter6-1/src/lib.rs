#![no_std]

use alloc::{format, string::String, vec::Vec};
extern crate alloc;

fn try_build_a_vec() -> Result<Vec<u8>, String> {
    let mut v = Vec::new();

    let required_size = 4;

    v.try_reserve(required_size)
        .map_err(|_e| format!("Failed to allocate {} items!", required_size))?;

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    Ok(v)
}

fn main() {}
