fn main() {
    let s1: String = String::from("Hello, World");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[1..3]);

    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("Josh"),
        age: 8,
    };

    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }

    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 10 };

    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    };

    let result2: Result<i32, String> = Ok(200);
    if let Ok(code) = result2 {
        println!("code: {}", code);
    }

    let result3: Result<i32, String> = Ok(200);
    println!("code: {}", result3.unwrap_or(-1));
    let result4: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result4.unwrap_or(-1));

    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }
    let result5: Result<i32, String> = Ok(200);
    let next_result = result5.and_then(func);
    let result6: Result<i32, String> = Err("error".to_string());
    let next_result2 = result6.and_then(func);

    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("code: {}", code);
        Ok(100)
    }

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    for element in &v1 {
        println!("{}", element);
    }

    let byte_array = [b'h', b'e', b'1', b'1', b'o'];
    print(Box::new(byte_array));

    fn print(s: Box<[u8]>) {
        println!("{:?}", s);
    }

    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }
    let result = if 0 <= number {
        number
    } else {
        -number
    };

    let mut count = 0;
    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    let mut count = 0;
    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    let count: i32;
    for count in 0..10 {
        println!("count: {}", count);
    }
    let array = [0, 1, 2];
    for element in &array {
        println!("element: {}", element);
    }

    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("sub loop end");
        }
        println!("main loop end");

        let i: i32 = 1;
        match i {
            1 => println!("1"),
            2 => println!("2"),
            3 => println!("3"),
            _ => println!("misc"),
        }
    }

    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    let result: Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        },
    };

    for number in 1..5 {
        println!("{}", number);
    }

    struct Iter {
        current: usize,
        max: usize
    }

    impl Iterator for Iter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.current += 1;
            if self.current - 1 < self.max {
                Some(self.current - 1)
            } else {
                None
            }
        }
    }

    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }

    let x = add(1, 2);
    println!("x = {}", x);

    struct Person2 {
        name: String,
        age: u32,
    }

    impl Person2 {
        fn new(name: &str, age: u32) -> Self {
            Self {
                name: String::from(name),
                age: age
            }
        }

        fn say_name(&self) -> &Self {
            println!("I am {}", self.name);
            self
        }

        fn say_age(&self) -> &Self {
            println!("I am {} year(s) old", self.age);
            self
        }
    }

    let p = Person2{
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name().say_age();

    let p = Person2::new("Taro", 20);
    p.say_name().say_age();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abs(number: i32) -> i32 {
    if number < 0 {
        return - number;
    }
    number
}
