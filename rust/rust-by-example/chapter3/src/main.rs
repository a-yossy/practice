use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn rect_area(rectangle: &Rectangle) -> f32 {
        let Rectangle {
            top_left:
                Point {
                    x: top_left_x,
                    y: top_left_y,
                },
            bottom_right:
                Point {
                    x: bottom_right_x,
                    y: bottom_right_y,
                },
        } = rectangle;

        (bottom_right_x - top_left_x) * (bottom_right_y - top_left_y)
    }

    fn square(point: &Point, length: f32) -> Rectangle {
        let Point {
            x: bottom_left_x,
            y: bottom_left_y,
        } = point;
        let top_left = Point {
            x: *bottom_left_x,
            y: bottom_left_y - length,
        };
        let bottom_right = Point {
            x: bottom_left_x + length,
            y: *bottom_left_y,
        };

        Rectangle {
            top_left,
            bottom_right,
        }
    }

    let point = Point { x: 10.3, y: 0.4 };
    let bottom_right = Point { x: 5.2, ..point };
    println!(
        "{}",
        rect_area(&Rectangle {
            top_left: Point { x: 3.1, y: 4.1 },
            bottom_right: Point { x: 6.1, y: 10.1 }
        })
    );
    println!("{:?}", square(&Point { x: 3.0, y: 4.0 }, 5.0));

    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    let x = Operations::Add;

    enum Number {
        Zero,
        One,
        Two,
    }

    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("{}", Number::Zero as i32);
    println!("{}", Number::One as i32);
    println!("{:06x}", Color::Red as i32);
    println!("{:06x}", Color::Blue as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("{}", list.len());
    println!("{}", list.stringify());
}
