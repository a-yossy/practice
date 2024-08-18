use proconio::input;

fn main() {
    input! {
        x_1: i32,
        x_2: i32,
        x_3: i32,
        x_4: i32,
        x_5: i32,
    }

    let ans = if x_1 == 0 {
        1
    } else if x_2 == 0 {
        2
    } else if x_3 == 0 {
        3
    } else if x_4 == 0 {
        4
    } else {
        5
    };
    println!("{}", ans);
}
