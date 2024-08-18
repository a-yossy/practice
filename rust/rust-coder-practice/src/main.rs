use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }

    let minute_hand = m / 60. * 2. * std::f64::consts::PI;
    let hour_hand = (h * 60. + m) / (12. * 60.) * 2. * std::f64::consts::PI;
    let angle = minute_hand - hour_hand;
    let squared = a * a + b * b  - 2. * a * b * angle.cos();

    println!("{}", squared.sqrt());
}
