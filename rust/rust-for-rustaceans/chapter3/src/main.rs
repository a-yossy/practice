use std::{borrow::Borrow, collections::HashSet, ops::Deref};

trait Printable {
    fn print(&self) -> String;
}

struct MyStruct {
    value: i32,
}

impl Printable for MyStruct {
    fn print(&self) -> String {
        format!("MyStruct value: {}", self.value)
    }
}

impl<T> Printable for &T
where
    T: Printable,
{
    fn print(&self) -> String {
        (*self).print()
    }
}

impl<T> Printable for &mut T
where
    T: Printable,
{
    fn print(&self) -> String {
        (**self).print()
    }
}

fn display<T: Printable>(item: T) {
    println!("{}", item.print());
}

impl<T> Printable for Box<T>
where
    T: Printable,
{
    fn print(&self) -> String {
        (**self).print()
    }
}

struct MyCollection {
    data: Vec<i32>,
}

impl MyCollection {
    fn new(data: Vec<i32>) -> Self {
        MyCollection { data }
    }
}

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a> IntoIterator for &'a MyCollection {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a> IntoIterator for &'a mut MyCollection {
    type Item = &'a mut i32;
    type IntoIter = std::slice::IterMut<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for MyBox<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for MyBox<T> {
    fn from(item: T) -> Self {
        MyBox(item)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn process_data_as_ref<T: AsRef<str>>(data: T) {
    println!("Processing data: {}", data.as_ref());
}

fn check_contains<T: Borrow<str>>(set: &HashSet<String>, value: T) {
    if set.contains(value.borrow()) {
        println!("Set contains: {}", value.borrow());
    } else {
        println!("Set does not contain: {}", value.borrow());
    }
}

struct MySmartPointer<T>
where
    T: std::fmt::Debug,
{
    value: T,
}

impl<T> MySmartPointer<T>
where
    T: std::fmt::Debug,
{
    fn new(value: T) -> Self {
        MySmartPointer { value }
    }

    fn frobnicate(&self) {
        println!("Frobnicate: {:?}", self.value);
    }
}

impl<T> Deref for MySmartPointer<T>
where
    T: std::fmt::Debug,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
struct MyData {
    id: u32,
}

impl MyData {
    fn frobnicate(&self) {
        println!("Frobnicate data: {:?}", self.id);
    }
}

fn main() {
    let my_struct = MyStruct { value: 42 };
    display(&my_struct);
    display(my_struct);

    let mut collection = MyCollection::new(vec![1, 2, 3, 4, 5]);
    for item in &collection {
        println!("Item: {}", item);
    }
    for item in &mut collection {
        *item += 10; // Modify the items
    }
    for item in collection {
        println!("Modified Item: {}", item);
    }

    let x = 5;
    let boxed_x = MyBox::new(x);
    assert_eq!(*boxed_x, 5);
    let m = MyBox::new(String::from("Hello, Rust!"));
    let m_ref = m.as_ref();
    process_data_as_ref(m_ref);
    hello(&m);
    let direct_string = String::from("Direct String");
    process_data_as_ref(direct_string);
    let my_string_from = MyBox::from(String::from("Hello from From!"));
    println!("From value: {}", my_string_from.0);

    let my_int: MyBox<i32> = 100.into();
    println!("MyBox value: {}", my_int.0);

    let mut my_set: HashSet<String> = HashSet::new();
    my_set.insert("Hello".to_string());
    my_set.insert("World".to_string());
    check_contains(&my_set, "Hello");
    check_contains(&my_set, String::from("World"));
    check_contains(&my_set, "Rust".to_string());

    let my_data = MyData { id: 1 };
    let my_pointer = MySmartPointer::new(my_data);
    my_pointer.frobnicate();
}
