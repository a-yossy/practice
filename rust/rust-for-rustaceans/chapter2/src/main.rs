use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{BuildHasher, Hash},
    sync::Arc,
};

#[repr(C)]
struct Foo {
    tiny: bool,  // 1
    normal: u32, // 4
    small: u8,   // 1
    long: u64,   // 8
    short: u16,  // 2
}

// impl String {
//     pub fn contains(&self, p: &dyn Pattern) -> bool {
//         p.is_contained_in(&*self)
//     }
// }

trait Printable: Sized {
    fn print_info(&self);
}

struct MyStruct {
    value: i32,
}

impl Printable for MyStruct {
    fn print_info(&self) {
        println!("Value: {}", self.value);
    }
}

// fn print_anything(item: &dyn Printable) {
//     item.print_info();
// }

trait CloneInfo {
    fn get_id(&self) -> u32;

    fn clone_self(&self) -> Self
    where
        Self: Sized;
}

struct Data {
    id: u32,
    name: String,
}

impl CloneInfo for Data {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn clone_self(&self) -> Self {
        Data {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

struct MyNumber(i32);

impl Display for MyNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyNumber: {}", self.0)
    }
}

// fn create_map_verbose<K, V, S>(keys: Vec<K>, values: Vec<V>) -> HashMap<K, V, S>
// where
//     K: Hash + Eq,
//     S: BuildHasher + Default,
// {
//     HashMap::new()
// }

#[derive(Clone)]
struct Shared<T> {
    data: Arc<T>,
}

#[derive(Debug)]
struct NotClone;

impl Debug for Vec<NotClone>
where
    for<'a> &'a Self: IntoIterator,
    for<'a> <&'a Self as IntoIterator>::Item: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

fn main() {
    let my_struct = MyStruct { value: 42 };
    my_struct.print_info();

    // let printable_trait_object: &dyn Printable = &my_struct;

    let original_data = Data {
        id: 1,
        name: String::from("Original"),
    };

    let info_object: &dyn CloneInfo = &original_data;
    println!("ID: {}", info_object.get_id());

    // let cloned_data_from_object = info_object.clone_self();

    let cloned_data = original_data.clone_self();
    println!("Cloned ID: {}", cloned_data.get_id());
    original_data.clone_self();

    let num = MyNumber(42);
    let s = num.to_string();
    println!("String representation: {}", s);

    let not_clone_arc = Arc::new(NotClone);
    let shared_instance = Shared {
        data: not_clone_arc,
    };

    let cloned_shared_intance = shared_instance.clone();
}
