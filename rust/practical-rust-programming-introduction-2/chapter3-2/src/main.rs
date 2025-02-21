use std::io::Write;

fn main() {
    println!("Hello, world!");
    println!("hello {}", "world");
    eprint!("hello {}", "error");
    eprintln!("hello");

    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, " is 123");
    dbg!(w);

    // panic!("it will panic");
    let v = vec![1, 2, 3];

    println!("defined in file: {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    assert!(true);
    assert_eq!(1, 1);
    assert_ne!(1, 0);
    // debug_assert!(false);
    debug_assert_eq!(1, 1);
    debug_assert_ne!(1, 0);

    enum Emotion {
        Anger,
        Happy,
    }

    trait Emotional {
        fn get_happy(&mut self) -> String;
        fn get_anger(&mut self) -> String;
        fn tell_state(&self) -> String;
    }

    struct HappyPerson {
        name: String,
        state: Emotion,
    }

    impl Emotional for HappyPerson {
        fn get_anger(&mut self) -> String {
            unimplemented!()
        }
        fn get_happy(&mut self) -> String {
            format!("{} is always happy.", self.name)
        }
        fn tell_state(&self) -> String {
            todo!()
        }
    }

    let mut p = HappyPerson{
        name: "Takashi".to_string(),
        state: Emotion::Happy,
    };
    println!("{}", p.get_happy());

    fn f(x: usize) -> &'static str{
        match x {
            n if n * n % 3 == 0 => "3n",
            n if n * n == 1 => "3n + 1 or 3n + 2",
            _ => unreachable!(),
        }
    }

    #[derive(Eq, PartialEq)]
    struct A(i32);

    #[derive(PartialEq, PartialOrd)]
    struct B(f32);

    #[derive(Copy, Clone)]
    struct C;

    #[derive(Clone)]
    struct D;

    #[derive(Debug)]
    struct E;

    #[derive(Default)]
    struct F;

    println!("{:?}", A(0) == A(1));
    println!("{:?}", B(1.0) > B(0.0));
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;
    let d0 = D;
    let _d1 = d0.clone();
    println!("{:?}", E);
    let _f = F::default();

    let mut x = vec![0.1, 0.5, 0.3];
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());

    trait Tweet {
        fn tweet(&self);
        fn tweet_twice(&self) {
            self.tweet();
            self.tweet();
        }

        fn shout(&self) {
            println!("Uoooooh!!!");
        }
    }

    struct Dove;
    struct Duck;
    impl Tweet for Dove {
        fn tweet(&self) {
            println!("Coo!")
        }
    }
    impl Tweet for Duck {
        fn tweet(&self) {
            println!("Quack!")
        }
    }
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();
    let duck = Duck {};
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
        (t, s)
    }

    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "World!");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");

    struct Color {
        r: i32,
        g: i32,
        b: i32,
    }
    let a = Color { r: 255, g: 255, b: 255 };
    let b = a;
    println!("{} {} {}", b.r, b.g, b.b);

    fn calc_data(data: String) -> String {
        println!("{}", data);
        data
    }
    let mut important_data = "Hello, World!".to_string();
    important_data = calc_data(important_data);
    println!("{}", important_data);

    fn calc_data2(data: &String) {
        println!("{}", data);
    }

    let important_data = "Hello, World!".to_string();
    calc_data2(&important_data);
    println!("{}", &important_data);

    let x = 5;
    let y = &x;
    let z  = &x;
    dbg!(x);
    dbg!(y);
    dbg!(z);

    let mut x = 5;
    // {
    //     let y = &mut x;
    //     let z = &mut x;
    //     dbg!(y);
    //     dbg!(z)
    // }
    // {
    //     let y = &x;
    //     let z = &mut x;
    //     dbg!(y);
    //     dbg!(z);
    // }

    // let y;
    // {
    //     let x = 5;
    //     y = &x;
    //     dbg!(x);
    // }
    // dbg!(y);

    let mut x = 5;
    let y = &x;
    let z = &mut x;
    dbg!(z);
    dbg!(x);

    struct Droppable;
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Resource will be released!");
        }
    }
    {
        let d = Droppable;
    }
    println!("The Droppable should be released at the end of block");
}
