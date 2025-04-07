use std::rc::Rc;

struct Person {
    name: Option<String>,
    birth: i32,
}

#[derive(Copy, Clone)]
struct Label {
    number: u32,
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

fn main() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("name1".to_string()),
        birth: 1525,
    });

    // let first_name = std::mem::replace(&mut composers[0].name, None);
    // assert_eq!(first_name, Some("nam1".to_string()));
    // assert_eq!(composers[0].name, None);
    let first_name = composers[0].name.take();

    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);

    let s: Rc<String> = Rc::new("string1".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
}
