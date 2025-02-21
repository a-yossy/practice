fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn assert_sample() {
    assert!(true);

    assert!(false, "panic! value={}", false);
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(true, false, "panic! value=({}, {})", true, false);
}

#[test]
#[should_panic]
fn test_panic() {
    panic!("expected panic");
}
