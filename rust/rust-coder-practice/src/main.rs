use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Chars
    }

    let mut flag = true;
    let mut count = 1;
    for c in s {
        if count % 2 == 0 {
            match c {
                'a'..='z' => flag = false,
                'A'..='Z' => {},
                _ => panic!()
            }
        } else {
            match c {
                'a'..='z' => {},
                'A'..='Z' => flag = false,
                _ => panic!()
            }
        }

        count += 1;
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
