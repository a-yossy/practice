fn func_of_func(b: i32) -> impl Fn(i32) -> i32 {
    move |x| x + b
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

fn print_point(p: Point) {
    println!("{:?}", p);
}

fn func_of_func2(b: i32, p: Point) -> impl FnOnce(i32) -> i32 {
    move |x| {
        print_point(p);
        x + b
    }
}

fn func_of_func3(b: i32) -> impl FnMut(i32) -> i32 {
    let mut count = 0;
    move |x| {
        count += 1;
        println!("count: {}", count);
        x + b
    }
}

fn insert(x: i32, xs: &[i32]) -> Vec<i32> {
    match xs {
        [y, ys @ ..] => {
            if x <= *y {
                [&[x][..], xs].concat()
            } else {
                [&[*y][..], &insert(x, ys)].concat()
            }
        },
        [] => vec![x],
    }
}

fn main() {
    let add_2 = func_of_func(2);
    println!("{}", add_2(1));
    println!("{}", add_2(4));

    let p = Point { x: 1.0, y: 2.0 };
    let add_2 = func_of_func2(2, p);
    println!("{}", add_2(1));

    let mut add_2 = func_of_func3(2);
    println!("{}", add_2(1));
    println!("{}", add_2(2));

    println!();

    let mut add_3 = func_of_func3(3);
    println!("{}", add_3(1));
    println!("{}", add_3(2));

    let v: Vec<i32> = Vec::new();
    let v = insert(2, &v);
    println!("{:?}", v);

    let v = insert(1, &v);
    println!("{:?}", v);

    let v = insert(3, &v);
    println!("{:?}", v);
}
