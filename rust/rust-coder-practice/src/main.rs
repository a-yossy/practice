use proconio::input;

fn main() {
    input! {
        n: i32,
        y:i32
    }

    for i in 0..=n {
        for j in 0..=n-i {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("{} {} {}", -1, -1, -1);
}
