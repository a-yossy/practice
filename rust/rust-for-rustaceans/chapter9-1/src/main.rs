// #![feature(dropck_eyepatch)]

// impl<T> SomeType<T> {
//     pub unsafe fn decr(&self) {
//         self.some_usize -= 1;
//     }
// }

// impl<T> SomeType<T> {
//     pub fn as_ref(&self) -> &T {
//         unsafe { &*self.ptr }
//     }
// }

use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit}, ptr::NonNull,
};

struct Person {
    name: *const str,
    age: usize,
}

struct Parsed {
    bytes: [u8; 1024],
    parsed: Person,
}

struct SelfRef<'a> {
    data: String,
    ptr_to_data: *const String,
    _phantom: std::marker::PhantomData<&'a Self>,
}

impl<'a> SelfRef<'a> {
    fn new(data: String) -> SelfRef<'a> {
        let mut s = SelfRef {
            data,
            ptr_to_data: std::ptr::null(),
            _phantom: std::marker::PhantomData,
        };
        s.ptr_to_data = &s.data as *const String;
        s
    }

    fn print_data_vir_ptr(&self) {
        unsafe {
            println!("Data: {}", *self.ptr_to_data);
        }
    }
}

// fn fill(gen: impl FnMut() -> Option<u8>) {
//     let mut buf = [MaybeUninit::<u8>::uninit(); 4096];
//     let mut last = 0;
//     for (i, g) in std::iter::from_fn(gen).take(4096).enumerate {
//         buf[i] = MaybeUninit::new(g);
//         last = i + 1;
//     }
//     let init: &[u8] = unsafe {
//         MaybeUninit::slice_assume_init_ref(&buf[..last])
//     };
// }

// impl<T: Default> Vec<T> {
//     pub fn fill_default(&mut self) {
//         let fill = self.capacity() - self.len();
//         if fill == 0 {
//             return;
//         }
//         let start = self.len();
//         unsafe {
//             self.set_len(start + fill);
//             for i in 0..fill {
//                 *self.get_unchecked_mut(start + i) = T::default();
//             }
//         }
//     }
// }

// struct Foo<T> {
//     one: bool,
//     two: PhantomData<T>,
// }
// struct Bar;
// struct Baz;
// type A = Foo<Bar>;
// type B = Foo<Baz>;

trait SneakyTrait {
    type Sneaky;
}
struct Wrapper<T: SneakyTrait> {
    item: T::Sneaky,
    iter: PhantomData<T>,
}
impl SneakyTrait for PhantomData<u8> {
    type Sneaky = ();
}
impl SneakyTrait for PhantomData<i8> {
    type Sneaky = [u8; 1024];
}

fn main() {
    let y = UnsafeCell::new(30);
    let r_y = &y;
    unsafe {
        *r_y.get() += 1;
    }

    println!("{:?}", unsafe { *r_y.get() });

    let original_box: Box<i32> = Box::new(10);
    let raw_ptr: *mut i32 = Box::into_raw(original_box);

    let box1 = ManuallyDrop::new(unsafe {
        Box::from_raw(raw_ptr);
    });
    let box2 = ManuallyDrop::new(unsafe {
        Box::from_raw(raw_ptr);
    });

    // let mut x = true;
    // let foo = Foo(&mut x);
    // x = false;

    // struct Foo<'a> {
    //     data: &'a mut bool,
    // }
    // impl<'a> Drop for Foo<'a> {
    //     fn drop(&mut self) {
    //         println!("Dropping Foo with data: {}", self.data);
    //     }
    // }
    // let mut x = true;
    // let foo = Foo { data: &mut x };
    // x = false;

    // fn barify<'a>(_: &'a mut i32) -> Bar<Foo<'a>> {
    //     ..
    // }
    // let mut x = true;
    // let foo = barify(&mut x);
    // x = false;

    // struct Foo<'a> {
    //     data: &'a mut bool,
    // }
    // impl<'a> Drop for Foo<'a> {
    //     fn drop(&mut self) {
    //         println!("Dropping Foo with data: {}", self.data);
    //     }
    // }
    // struct Bar<T> {
    //     inner: T,
    // }
    // fn barify<'a>(data: &'a mut bool) -> Bar<Foo<'a>> {
    //     Bar {
    //         inner: Foo { data },
    //     }
    // }
    // let mut x = true;
    // let foo = barify(&mut x);
    // x = false;

    struct Foo<'a> {
        data: &'a mut bool,
    }
    impl<'a> Drop for Foo<'a> {
        fn drop(&mut self) {
            println!("Dropping Foo with data: {}", self.data);
        }
    }
    use std::marker::PhantomData;
    struct Bar<T> {
        _phantom: PhantomData<T>,
    }
    fn barify<'a>(data: &'a mut bool) -> Bar<Foo<'a>> {
        Bar {
            _phantom: PhantomData,
        }
    }
    let mut x = true;
    let foo = barify(&mut x);
    x = false;

    // struct MyBox<T>(T);
    // impl<T> Drop for MyBox<T> {
    //     fn drop(&mut self) {
    //         println!("Dropping MyBox with data");
    //     }
    // }
    // let mut x = true;
    // let my_box_ref = MyBox(&mut x);
    // x = false;

    // struct MyBox<T>(T);
    // unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
    //     fn drop(&mut self) {
    //         println!("Dropping MyBox with data");
    //     }
    // }
    // let mut x = true;
    // let my_box_ref = MyBox(&mut x);
    // x = false;

    // use std::ptr;
    // struct MyRawBox<T> {
    //     ptr: *mut T,
    // }
    // impl<T> MyRawBox<T> {
    //     fn new(data: T) -> Self {
    //         let boxed = Box::new(data);
    //         MyRawBox {
    //             ptr: Box::into_raw(boxed),
    //         }
    //     }
    // }
    // unsafe impl<#[may_dangle] T> Drop for MyRawBox<T> {
    //     fn drop(&mut self) {
    //         println!("Dropping MyRawBox with data");
    //     }
    // }
    // let mut x = String::from("Hello, world!");
    // let raw_box_ref = MyRawBox::new(&mut x);
    // x = String::from("Goodbye, world!");

    // struct Box<T> {
    //     t: NonNull<T>,
    //     _owned: PhantomData<T>,
    // }
    // unsafe impl<#[mya_dangle] T> Drop for Box<T> {
    //     fn drop(&mut self) {
    //         println!("Dropping Box with data");
    //     }
    // }
}
