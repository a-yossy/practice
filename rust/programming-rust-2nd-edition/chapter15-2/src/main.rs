use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList},
    error::Error,
    io::BufRead,
    str::FromStr,
};

fn all<I, P>(mut iter: I, mut predicate: P) -> bool
where
    I: Iterator + Sized,
    P: FnMut(I::Item) -> bool,
{
    use std::ops::ControlFlow::*;
    iter.try_fold((), |_, item| {
        if predicate(item) {
            Continue(())
        } else {
            Break(())
        }
    }) == Continue(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // let stdin = std::io::stdin();
    // println!("{}", stdin.lock().lines().count());

    fn triangle(n: u64) -> u64 {
        (1..=n).sum()
    }
    assert_eq!(triangle(20), 210);

    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }
    assert_eq!(factorial(20), 2432902008176640000);

    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));

    fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
        lhs.partial_cmp(rhs).unwrap()
    }
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));

    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);
    assert_eq!(
        populations.iter().max_by_key(|&(_name, pop)| pop),
        Some((&"Portland", &583_776))
    );
    assert_eq!(
        populations.iter().min_by_key(|&(_name, pop)| pop),
        Some((&"Greenhorn", &2))
    );

    let packed = "Helen of Troy";
    let spaced = "Helen   of    Troy";
    let obscure = "Helen of Sandusky";
    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    assert!(spaced < obscure);
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));

    let id = "Iterator";
    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));

    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);

    let bytes = b"Xerxes";
    assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    assert_eq!(bytes.iter().rposition(|&c| c == b'X'), Some(0));

    let a = [5, 6, 7, 8, 9, 10];
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200);
    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max), 10);

    let a = [
        "Pack", "my", "box", "with", "five", "dozen", "liquor", "jugs",
    ];
    let pangram = a.iter().fold(String::new(), |s, w| s + w + " ");
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs ");
    let weird_pangram = a.iter().rfold(String::new(), |s, w| s + w + " ");
    assert_eq!(weird_pangram, "jugs liquor dozen five with box my Pack ");

    // let stdin = std::io::stdin();
    // let sum = stdin
    //     .lock()
    //     .lines()
    //     .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
    //         Ok(sum + u64::from_str(&line?.trim())?)
    //     })?;
    // println!("{}", sum);

    let mut squares = (0..10).map(|i| i * i);
    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.nth(6), None);

    let squares = (0..10).map(|i| i * i);
    assert_eq!(squares.last(), Some(81));

    assert_eq!(
        populations.iter().find(|&(_name, &pop)| pop > 1_0000_000),
        None
    );
    assert_eq!(
        populations.iter().find(|&(_name, &pop)| pop > 500_000),
        Some((&"Portland", &583_776))
    );

    let args: HashSet<String> = std::env::args().collect();
    let args: BTreeSet<String> = std::env::args().collect();
    let args: LinkedList<String> = std::env::args().collect();
    let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
    let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();

    // let mut vec =  Vec::new();
    // for item in iter {
    //     vec.push(item);
    // }

    let mut v: Vec<i32> = (0..5).map(|i| 1 << i).collect();
    v.extend([31, 57, 99, 163]);
    assert_eq!(v, [1, 2, 4, 8, 16, 31, 57, 99, 163]);

    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
    let (living, nonliving): (Vec<&str>, Vec<&str>) =
        things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);
    assert_eq!(living, vec!["mushroom", "giraffe", "grapefruit"]);
    assert_eq!(nonliving, vec!["doorknob", "noodle"]);

    ["doves", "hens", "birds"]
        .iter()
        .zip(["turtle", "french", "calling"])
        .zip(2..5)
        .rev()
        .map(|((item, kind), quantity)| format!("{} {} {}", quantity, kind, item))
        .for_each(|gift| {
            println!("You have received: {}", gift);
        });

    Ok(())
}
