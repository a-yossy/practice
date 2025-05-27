#![feature(trace_macros)]

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(
                        "assertion failed: `(left == right)` \
                        (left: `{:?}`, right: `{:?}`)",
                        left_val, right_val
                    )
                }
            }
        }
    }};
}

// macro_rules! vec {
//     ($elem:expr ; $n:expr) => {
//         ::std::vec::from_elem($elem, $n)
//     };
//     ($($x:expr),*) => {
//         // <[_]>::into_vec(Box::new([$($x),*]))
//         {
//             let mut v = Vec::new();
//             $(v.push($x);)*
//             v
//         }
//     };
//     ($($x:expr),+,) => {
//         vec![$($x),*]
//     };
// }

fn main() {
    trace_macros!(true);
    let numbers = vec![1, 2, 3];
    trace_macros!(false);
    println!("total: {}", numbers.iter().sum::<u64>());
}
