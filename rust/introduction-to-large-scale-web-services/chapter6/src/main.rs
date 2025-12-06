use std::{collections::VecDeque, fs::File, io::Read};

use anyhow::Result;

fn main() -> Result<()> {
    let mut f = File::open("temp.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("{contents}");

    let temp = encode_vb(&vec![5, 130]);

    for n in temp {
        println!("n: {:?}", n);
        println!("decode_vb: {:?}", decode_vb(n));
    }

    Ok(())
}

fn encode_vb(numbers: &[i32]) -> Vec<VecDeque<i32>> {
    let mut bytestream = Vec::new();
    for n in numbers {
        let mut number = *n;
        let mut bytes = VecDeque::new();
        loop {
            bytes.push_front(number % 128);
            if number < 128 {
                break;
            }
            number = number / 128;
        }
        let last_index = bytes.len() - 1;
        bytes[last_index] += 128;

        bytestream.push(bytes);
    }

    bytestream
}

fn decode_vb(bytestream: VecDeque<i32>) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut n = 0;
    for i in 0..bytestream.len() {
        if bytestream[i] < 128 {
            n = 128 * n + bytestream[i];
        } else {
            n = 128 * n + (bytestream[i] - 128);
            numbers.push(n);
            n = 0;
        }
    }

    numbers
}
