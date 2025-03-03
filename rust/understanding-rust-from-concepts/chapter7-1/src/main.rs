fn main() {
    let mut r = 1..3;

    println!("{:?}", r.next());
    println!("{:?}", r.next());
    println!("{:?}", r.next());

    let vv = vec![1, 2, 3, 4];
    let mut iter = vv.into_iter();

    let x = iter.next().unwrap();
    println!("{}", x);

    let x = iter.next().unwrap();
    println!("{}", x);

    let vv = vec![1, 2, 3, 4];
    let mut iter = (&vv).into_iter();

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = vv[2];
    println!("{}", x);

    let mut vv = vec![1, 2, 3, 4];
    let mut iter = (&mut vv).into_iter();

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = iter.next().unwrap();
    println!("{}", *x);

    *x += 10;
    println!("{:?}", vv);

    {
        let values = vec![1, 2, 3, 4];
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                let next;
                match iter.next() {
                    Some(val) => next = val,
                    None => break,
                };
                let x = next;
                let () = { println!("{}", x); };
            }
        };

        result
    }

    let vv = vec![0, 1, 2, 3, 4];
    for ii in &vv {
        println!("{}", ii);
    }
    for ii in &vv {
        println!("{}", ii);
    }

    let c = "あいうえお";
    let c_vec = c.chars().collect::<Vec<_>>();
    println!("{:?}", c_vec);

    fn fact(n: u32) -> u32 {
        let mut ans = n;
        for ii in (1..=(n - 1)).rev() {
            ans *= ii;
        }
        ans
    }
    fn fact2(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * fact2(n - 1)
        }
    }

    fn add_one(x: i32) -> i32 {
        x + 1
    }
    let v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();
    let v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(|x: i32| -> i32 { x + 1 }).collect();
    let add_one = |x| x + 1;
    let v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();

    let m = 1;
    let add_m = |x: i32| x + m;
    println!("{}", add_m(1));

    let m = 1;
    let add_m = |x: i32| x + m;
    println!("{}", add_m(1));

    let  m = 10;
    let add_m = |x: i32| x + m;
    println!("{}", add_m(1));

    let m = 1;
    let add_m = |x: i32| x + m;
    println!("{}", add_m(1));

    let m = 10;
    println!("{}", add_m(1));

    let v = vec![0, 1, 2, 3];
    let filtered: Vec<i32> = v.into_iter().filter(|&x| x >= 2).collect();
    println!("{:?}", filtered);

    let w = vec![4, 5, 6];
    let sum = (&w).into_iter().fold(0, |x, y| x + y);
    println!("{}", sum);
    let sum = w.into_iter().reduce(|x, y| x + y).unwrap();
    println!("{}", sum);
}
