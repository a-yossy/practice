use proconio::input;

fn main() {
    input! {
        mut l: [i32]
    }

    l.sort();
    let mut ans = 0;
    for k in 0..l.len() {
        for j in 0..k {
            for i in 0..j {
                if l[i] != l[j] && l[j] != l[k] && l[k] != l[i] && l[i] + l[j] > l[k] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
