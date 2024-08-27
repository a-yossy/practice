use proconio::input;

fn main() {
    input! {
        n_d: (usize, usize),
        x_y: [(i64, i64); n_d.0],
    }

    let mut ans = 0;
    for (x, y) in &x_y {
        if ((x * x + y * y) as f64).sqrt() <= n_d.1 as f64 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
