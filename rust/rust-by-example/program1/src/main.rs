use std::fmt;

fn main() {
    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}");

    struct UnPrintable(i32);
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);
    println!("{:?}", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);
    println!("{:#?}", peter);

    struct Structure2(i32);
    impl fmt::Display for Structure2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}, {}", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64
    }
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);
    println!("{}", minmax);
    println!("{:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("{}", point);
    println!("{:?}", point);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("{}", complex);
    println!("{:?}", complex);
}
