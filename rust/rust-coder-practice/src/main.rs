use proconio::input;

fn main() {
    input! {
        n: String
    }

    let mut sum = 0;
    for c in n.chars() {
        sum += (c.to_string()).parse::<i32>().unwrap();
    }

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
