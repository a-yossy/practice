fn main() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;

    let string = "Hello, world!";

    let mut x;
    x = 42; // 1
    let y = &x; // 2
    // x = 43; // 3
    assert_eq!(*y, 42); // 4

    let x1 = 42;
    let y1 = Box::new(84);
    let z = (x1, y1);
    let x2 = x1;
    // let y2 = y1;

    fn cache(input: &i32, sum: &mut i32) {
        *sum = *input + *input;
        assert_eq!(*sum, 2 * *input);
    }

    fn noalias(input: &i32, output: &mut i32) {
        if *input == 1 {
            *output = 2;
        }
        if *input != 1 {
            *output = 3;
        }
    }

    let x = 42;
    let mut y = &x;
    let z = &mut y;

    fn replace_with_84(s: &mut Box<i32>) {
        // let was = *s;
        let was = std::mem::take(s);
        *s = was;
        let mut r = Box::new(84);
        std::mem::swap(s, &mut r);
        assert_eq!(*r, 84);
    }
    let mut s = Box::new(42);
    // replace_with_84(&mut s);

    let mut x = Box::new(42);
    let r = &x;
    if true {
        *x = 84;
    } else {
        println!("{}", r);
    }

    let mut x = Box::new(42);
    let mut z = &x;
    for i in 0..100 {
        println!("{}", z);
        x = Box::new(i);
        z = &x;
    }
    println!("{}", z);

    struct StrSplit<'s, 'p> {
        delimiter: &'p str,
        document: &'s str,
    }
    impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
        type Item = &'s str;
        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }
    fn str_before(s: &str, c: char) -> Option<&str> {
        StrSplit {
            document: s,
            delimiter: &c.to_string(),
        }
        .next()
    }

    struct MutStr<'a, 'b> {
        s: &'a mut &'b str,
    }
    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    println!("{}", s);
}
