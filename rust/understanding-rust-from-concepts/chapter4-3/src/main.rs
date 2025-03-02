use std::rc::Rc;

struct TestA {
    data_i32: i32,
    data_string: String,
}

struct DataA {
    number_a: Option<Rc<i32>>
}

struct DataB {
    number_b: Option<Rc<i32>>
}

fn setdata(data_a: &mut DataA, data_b: &mut DataB, value: i32) {
    let number = Rc::new(value + 10);
    data_a.number_a = Some(Rc::clone(&number));
    data_b.number_b = Some(Rc::clone(&number));
}

fn main() {
    let mut data_a_1 = DataA { number_a: None };
    let mut data_b_1 = DataB { number_b: None };
    let mut data_a_2 = DataA { number_a: None };
    let mut data_b_2 = DataB { number_b: None };

    setdata(&mut data_a_1, &mut data_b_1, 1);
    setdata(&mut data_a_2, &mut data_b_2, 2);

    let x = Rc::new(TestA {
        data_i32: 1,
        data_string: String::from("Hello"),
    });
    let data_i32 = &x.data_i32;
    let data_string = &x.data_string;
}
