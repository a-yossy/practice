use std::{default, fmt::Debug, iter::Cloned, sync::Mutex};

fn main() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum MyBooleanOption {
        Off,
        On,
    }

    #[derive(Debug, Clone)]
    struct KeyId(u32);
    let k = KeyId(42);
    let k2 = k;
    // println!("k = {k:?}");

    #[derive(Debug, Clone, Copy)]
    struct KeyId2(u32);
    let k = KeyId2(42);
    let k2 = k;
    println!("k = {k:?}");
    let k3 = k.clone();

    #[derive(Default)]
    enum IceCreamFlavor {
        Chocolate,
        Strawberry,
        #[default]
        Vanilla,
    }

    #[derive(Default)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    }
    let c = Color {
        red: 128,
        ..Default::default()
    };

    #[derive(PartialEq, PartialOrd)]
    struct Oddity(f32);
    let x = Oddity(f32::NAN);
    let y = Oddity(f32::NAN);
    if x <= x {
        println!("This line does not get executed!");
    }

    if x <= y {
        println!("y is bigger");
    } else if y < x {
        println!("x is bigger");
    } else {
        println!("Neither is bigger");
    }

    struct ThreadSafeInt {
        value: Mutex<i32>,
    }
    impl ThreadSafeInt {
        fn new(val: i32) -> Self {
            Self {
                value: Mutex::new(val),
            }
        }
        fn add(&self, delta: i32) {
            let mut v = self.value.lock().unwrap();
            *v += delta;
        }
        fn add_with_extras(&self, delta: i32) {
            {
                let mut v = self.value.lock().unwrap();
                *v += delta;
            }
        }
    }

    #[derive(Debug)]
    struct MyStruct(i32);
    impl Drop for MyStruct {
        fn drop(&mut self) {
            println!("Dropping {self:?}");
        }
    }
    let x = MyStruct(42);
    // {
    //     x.drop();
    //     x.0 += 1;
    // }
    {
        drop(x);
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Point {
        x: i64,
        y: i64,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Bounds {
        top_left: Point,
        bottom_right: Point,
    }

    fn overlap(a: Bounds, b: Bounds) -> Option<Bounds> {
        todo!()
    }

    pub trait Draw {
        fn bounds(&self) -> Bounds;
    }

    pub fn on_screen<T>(draw: &T) -> bool
    where
        T: Draw,
    {
        overlap(draw.bounds(), draw.bounds()).is_some()
    }

    pub fn on_screen2<T: Draw>(draw: &T) -> bool {
        todo!()
    }
    pub fn on_screen3(draw: &impl Draw) -> bool {
        todo!()
    }

    #[derive(Clone)]
    struct Square {
        top_left: Point,
        size: i64,
    }
    impl Draw for Square {
        fn bounds(&self) -> Bounds {
            Bounds {
                top_left: self.top_left,
                bottom_right: Point {
                    x: self.top_left.x + self.size,
                    y: self.top_left.y + self.size,
                },
            }
        }
    }
    let square = Square {
        top_left: Point { x: 0, y: 0 },
        size: 10,
    };
    let visible = on_screen(&square);

    #[derive(Clone, Debug)]
    struct Circle {
        center: Point,
        radius: i64,
    }
    impl Draw for Circle {
        fn bounds(&self) -> Bounds {
            Bounds {
                top_left: Point {
                    x: self.center.x - self.radius,
                    y: self.center.y - self.radius,
                },
                bottom_right: Point {
                    x: self.center.x + self.radius,
                    y: self.center.y + self.radius,
                },
            }
        }
    }
    let circle = Circle {
        center: Point { x: 5, y: 5 },
        radius: 3,
    };
    let visible = on_screen(&circle);

    let square = Square {
        top_left: Point { x: 0, y: 0 },
        size: 10,
    };
    let draw: &dyn Draw = &square;

    pub fn on_screen4(draw: &dyn Draw) -> bool {
        overlap(draw.bounds(), draw.bounds()).is_some()
    }
    let visible = on_screen4(&square);
    let visible = on_screen4(&circle);

    fn area<T>(draw: &T) -> i64
    where
        T: Draw,
    {
        let bounds = draw.bounds();
        (bounds.bottom_right.x - bounds.top_left.x) * (bounds.bottom_right.y - bounds.top_left.y)
    }

    fn show<T>(draw: &T)
    where
        T: Debug + Draw,
    {
        println!("{:?} has bounds {:?}", draw, draw.bounds());
    }

    let square = Square {
        top_left: Point { x: 0, y: 0 },
        size: 10,
    };
    let circle = Circle {
        center: Point { x: 5, y: 5 },
        radius: 3,
    };
    println!("Square area: {}", area(&square));
    println!("Circle area: {}", area(&circle));
    show(&circle);
    // show(&square);

    trait Shape: Draw {
        fn render_in(&self, bounds: Bounds);

        fn render(&self) {
            if let Some(visible) = overlap(self.bounds(), self.bounds()) {
                self.render_in(visible);
            }
        }
    }
    impl Shape for Square {
        fn render_in(&self, bounds: Bounds) {
            println!("Rendering square in {:?}", bounds);
        }
    }
    impl Shape for Circle {
        fn render_in(&self, bounds: Bounds) {
            println!("Rendering circle in {:?}", bounds);
        }
    }

    trait Stamp: Draw {
        fn make_copy(&self) -> Self
        where
            Self: Sized;
    }
    let square = Square {
        top_left: Point { x: 0, y: 0 },
        size: 10,
    };
    impl Stamp for Square {
        fn make_copy(&self) -> Self {
            self.clone()
        }
    }
    let copy = square.make_copy();
    let stamp: &dyn Stamp = &square;
    // let copy = stamp.make_copy();

    let shapes: Vec<&dyn Shape> = vec![&square, &circle];
    for shape in shapes {
        shape.render();
    }

    // fn cloned<'a, T>(self) -> Cloned<Self>
    // where
    //     T: 'a + Clone,
    //     Self: Sized + Iterator<Item = &'a T>,
    // {
    //     Cloned::new(self)
    // }
}
