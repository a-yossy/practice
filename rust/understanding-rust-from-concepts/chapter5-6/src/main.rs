use std::ops::Deref;

struct MyData {
    x: i32
}

impl Deref for MyData {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

fn deref_data(x: &i32) -> i32 {
    *x
}

fn print_string(s: &str) {
    println!("{}", s);
}

fn main() {
    let data = MyData { x: 1 };
    let data_deref1 = data.deref();
    let data_deref2 = *(data.deref());
    let data_deref3 = *data;
    let data_deref4 = deref_data(&data);
    let data_deref5 = deref_data(&*&data);

    let boxed = Box::new(String::from("Hello"));
    print_string(&*boxed);
    print_string(&boxed);
}
