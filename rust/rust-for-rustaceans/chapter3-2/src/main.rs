use std::borrow::Cow;

fn main() {
    fn frobnicate1(s: String) -> String {
        s
    }
    fn frobnicate2(s: &str) -> Cow<'_, str> {
        Cow::Borrowed(s)
    }
    fn frobnicate3(s: impl AsRef<str>) -> impl AsRef<str> {
        s
    }

    fn print_anything_generic<T: std::fmt::Display>(s: T) {
        println!("{}", s);
    }

    let my_number = 42;
    let my_string = "Hello, world!".to_string();
    print_anything_generic(my_number);
    print_anything_generic(my_string);
    print_anything_generic(&"world");
    let boxed_number: Box<dyn std::fmt::Display> = Box::new(11);
    print_anything_generic(boxed_number);
    let some_string = "This is a string".to_string();
    let dynamic_ref: &dyn std::fmt::Display = &some_string;
    print_anything_generic(dynamic_ref);

    fn foo(v: impl AsRef<[usize]>) {
        println!("Length: {}", v.as_ref().len());
    }
    let numbers = vec![1, 2, 3];
    foo(&numbers);

    trait Drawable {
        fn draw(&self);
    }

    struct Circle;
    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing a circle");
        }
    }

    struct Square;
    impl Drawable for Square {
        fn draw(&self) {
            println!("Drawing a square");
        }
    }

    let shapes: Vec<Box<dyn Drawable>> = vec![Box::new(Circle), Box::new(Square)];
    for shape in shapes {
        shape.draw();
    }

    trait NotObjectSafe {
        fn do_something(&self) -> Self;
    }

    trait MyFormatter {
        fn format<T: ToString>(&self, value: T) -> String;
    }
    struct SimpleFormatter;
    impl MyFormatter for SimpleFormatter {
        fn format<T: ToString>(&self, value: T) -> String {
            format!("Formatted: {}", value.to_string())
        }
    }

    trait DrawSafe {
        fn draw(&self);
        fn draw_colored<C>(&self, color: C)
        where
            Self: Sized,
            C: AsRef<str>,
        {
            println!("Drawing with color: {}", color.as_ref());
            self.draw();
        }
    }

    impl DrawSafe for Circle {
        fn draw(&self) {
            println!("Drawing a circle");
        }
    }

    let circle = Circle;
    circle.draw_colored("red");

    let shapes: Vec<Box<dyn DrawSafe>> = vec![Box::new(Circle)];
    for shape in shapes {
        shape.draw();
        // shape.draw_colored("blue");
    }

    trait MyDroppable {}
    struct MyData(String);
    impl Drop for MyData {
        fn drop(&mut self) {
            println!("Dropping MyData with value: {}", self.0);
        }
    }
    struct DeferredDropQueue {
        items: Vec<Box<dyn Drop>>,
    }
    impl DeferredDropQueue {
        fn new() -> Self {
            DeferredDropQueue { items: Vec::new() }
        }

        fn add<T: Drop + 'static>(&mut self, item: T) {
            self.items.push(Box::new(item));
        }
        fn drop_all(&mut self) {
            println!("Dropping all items in the queue");
            self.items.clear();
        }
    }

    let mut queue = DeferredDropQueue::new();
    queue.add(MyData("First".to_string()));
    queue.add(vec![1, 2, 3]); // This will

    queue.drop_all();

    fn process_data(input: &str) -> Cow<str> {
        if input.contains("bad_word") {
            let processed = input.replace("bad_word", "good_word");
            Cow::Owned(processed)
        } else {
            Cow::Borrowed(input)
        }
    }

    let data1 = "Hello, this is a test.";
    let result1 = process_data(data1);
    println!("Processed data1: {}", result1);
    let data2 = "This contains a bad_word that needs to be replaced.";
    let result2 = process_data(data2);
    println!("Processed data2: {}", result2);
}
