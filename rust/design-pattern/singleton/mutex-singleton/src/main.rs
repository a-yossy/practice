use std::sync::Mutex;

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    {
        let array = ARRAY.lock().unwrap();
        println!("Called {} times: {:?}", array.len(), array);
    }

    {
        let mut array = ARRAY.lock().unwrap();
        *array = vec![3, 4, 5];
    }

    println!("New singleton object: {:?}", ARRAY.lock().unwrap());
}
