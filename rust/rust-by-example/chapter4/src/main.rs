// Original JavaScript code:
/*
const inputs = require("fs")
  .readFileSync("/dev/stdin", "utf8")
  .trim()
  .split("\n");
let index_i = 0;
const dh = [-1, 0, 1, 0];
const dw = [0, -1, 0, 1];

const N = +inputs[0];
const S = inputs[1].split("").map(Number);

const array = Array.from({ length: 10 }, () => 0);
for (let i = 0; i < N; i++) {
  array[S[i]]++;
}

let ans = 0;
for (let i = 0; ; i++) {
  const square = i * i;
  const square_str = String(square);
  if (square_str.length > S.length) break;
  const square_array = Array.from({ length: 10 }, () => 0);
  if (square_str.length > N) continue;
  if (square_str.length < N) square_array[0] += N - square_str.length;
  for (let j = 0; j < square_str.length; j++) {
    square_array[+square_str[j]]++;
  }

  if (JSON.stringify(array) === JSON.stringify(square_array)) ans++;
}

console.log(ans);
*/

// Translated Rust code:
use std::io::{self, Read};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut inputs = input.trim().split('\n');
    let n: usize = inputs.next().unwrap().parse().unwrap();
    let s_line = inputs.next().unwrap();
    let s_chars: Vec<u64> = s_line.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // Initialize array with digit counts from input
    let mut array = vec![0; 10];
    for &digit in &s_chars {
        array[digit as usize] += 1;
    }

    let mut ans = 0;
    let mut i = 0;

    loop {
        let square = i * i;
        let square_str = square.to_string();
        if square_str.len() > s_chars.len() {
            break;
        }

        if square_str.len() > n {
            i += 1;
            continue;
        }

        let mut square_array = vec![0; 10];
        if square_str.len() < n {
            square_array[0] += n - square_str.len();
        }

        for ch in square_str.chars() {
            square_array[ch.to_digit(10).unwrap() as usize] += 1;
        }

        if array == square_array {
            ans += 1;
        }
        i += 1;
    }

    println!("{}", ans);
}
