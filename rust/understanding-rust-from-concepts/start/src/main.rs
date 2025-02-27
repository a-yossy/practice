mod module_hello;
mod module_a;

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
}
