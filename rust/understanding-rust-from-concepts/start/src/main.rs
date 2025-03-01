mod module_hello;
mod module_a;
use std::cmp::Ordering;

fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 {
        None
    } else {
        Some(x / y)
    };
    ans
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn func_ex_print_somea<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_some_match2<T>(ans: Option<T>)
where
    T: std::fmt::Display
{
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_some_match3(ans: Option<impl std::fmt::Display>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}

fn myclear(x: &mut String) {
    x.clear();
}

fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn pick2<'a, 'b>(x: &'a [i32], y: &'a [i32], end: usize) -> (&'a [i32], &'a [i32]) {
    (&x[..end], &y[..end])
}

fn main() {
    module_hello::print_hello();
    crate::module_hello::print_hello();
    let mut s = "Hello".to_string();
    println!("s = {}", s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s = {}", s);
    let x = 1;
    println!("{:p}", &x);
    let arr = [1, 2, 3, 4];
    let x = &arr[..2];

    let x = 1;
    println!("{}", x);
    {
        let x = 2;
        println!("{}", x);
    }
    println!("{}", x);
    let tup = (1, 2);

    let ary = [0, 1, 2, 3];
    let ary_sliced  = &ary[0..2];

    let v = vec![0, 1, 2, 3];
    let v_sliced = &v[3..];

    let s = "hello";
    println!("{}", s);
    let s_slice = &s[0..2];
    let st = "あいうえお";
    let st_slice = &st[0..6];

    let mut st = String::from("あいうえお");
    let st_slice = &st[0..6];

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        fn new(name: String, age: u8) -> Person {
            Person{name, age}
        }

        fn age_incr(&self, incr: u8) -> u8 {
            self.age + incr
        }

        fn age_incr_replace(&mut self, incr: u8) {
            self.age += incr;
        }
    }

    #[derive(Debug)]
    struct Parents<'a, 'b> {
        father: &'a Person,
        mother: &'b Person,
    }

    impl<'a, 'b> Parents<'a, 'b> {
        fn new(father: &'a Person, mother: &'b Person) -> Parents<'a, 'b> {
            Parents { father, mother }
        }

        fn get_mother(&self) -> &Person {
            &self.father
        }
    }

    let taro = Person { name: String::from("taro"), age: 50 };
    let hanako = Person { name: String::from("hanako"), age: 48 };
    let sato = Parents::new(&taro, &hanako);
    println!("{:?}", sato);
    println!("{:?}", taro);

    enum Sign {
        Positive,
        Zero,
        Negative,
    }

    fn determine_sign(x: i32) -> Sign {
        match x.cmp(&0) {
            Ordering::Greater => Sign::Positive,
            Ordering::Less => Sign::Negative,
            Ordering::Equal => Sign::Zero,
        }
    }

    enum EnumExample {
        TupleTypeExample1(String),
        TupleTypeExample2(i32, bool),
        StructTypeExample{name: String, age: u8},
    }

    let mut v: Vec<EnumExample> = Vec::new();

    v.push(EnumExample::TupleTypeExample1(String::from("Hello")));
    v.push(EnumExample::TupleTypeExample2(10, true));
    v.push(EnumExample::StructTypeExample { name: String::from("taro"), age: 10 });

    for e in &v {
        if let EnumExample::StructTypeExample { name: n, age: a } = e {
            println!("StructTypeExample_iflet: name = {}, age = {}", n, a);
        }
    }

    for e in &v {
        match e {
            EnumExample::TupleTypeExample1(s) => {
                println!("TupleTypeExample1:s = {}", s);
            }
            EnumExample::TupleTypeExample2(n, b) => {
                println!("TupleTypeExample2: n = {}, b = {}", n, b);
            }
            EnumExample::StructTypeExample { name: n, .. } => {
                println!("StructTypeExample: name = {}", n);
            }
        }
    }

    fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
        if ans.is_some() {
            println!("{}", ans.unwrap())
        } else {
            println!("None")
        }
    }
}
