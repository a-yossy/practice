use ref_with_flag::RefWithFlag;

mod ref_with_flag {
    use std::marker::PhantomData;
    use std::mem::align_of;

    pub struct RefWithFlag<'a, T: 'a> {
        ptr_and_bit: usize,
        behaves_like: PhantomData<&'a T>,
    }

    impl<'a, T: 'a> RefWithFlag<'a, T> {
        pub fn new(ptr: &'a T, flag: bool) -> Self {
            assert!(align_of::<T>() % 2 == 0);
            Self {
                ptr_and_bit: ptr as *const T as usize | flag as usize,
                behaves_like: PhantomData,
            }
        }

        pub fn get_ref(&self) -> &'a T {
            unsafe {
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }

        pub fn get_flag(&self) -> bool {
            self.ptr_and_bit & 1 != 0
        }
    }
}

fn offset<T>(ptr: *const T, count: isize) -> *const T
where
    T: Sized,
{
    let bytes_per_element = std::mem::size_of::<T>() as isize;
    let byte_offset = count * bytes_per_element;
    (ptr as isize).checked_add(byte_offset).unwrap() as *const T
}

fn main() {
    let vec = vec![10, 20, 30];
    let flagged = RefWithFlag::new(&vec, true);
    assert_eq!(flagged.get_ref()[1], 20);
    assert_eq!(flagged.get_flag(), true);

    let flagged = RefWithFlag::new(&vec, false);
    assert_eq!(flagged.get_ref()[1], 20);
    assert_eq!(flagged.get_flag(), false);

    assert_eq!(std::mem::size_of::<i64>(), 8);
    assert_eq!(std::mem::align_of::<(i32, i32)>(), 4);

    let slice: &[i32] = &[1, 4, 9, 27, 81];
    assert_eq!(std::mem::size_of_val(slice), 20);

    let text: &str = "alligator";
    assert_eq!(std::mem::size_of_val(text), 9);

    use std::fmt::Display;
    let unremarkable: &dyn Display = &193_u8;
    let remarkable: &dyn Display = &0.0072973525664;

    assert_eq!(std::mem::size_of_val(unremarkable), 1);
    assert_eq!(std::mem::align_of_val(remarkable), 8);

    let pot = "pasta".to_string();
    let plate = pot;

    let mut noodles = vec!["udon".to_string()];
    let soda = "soda".to_string();
    noodles.push(soda);
    let last;
    last = noodles.pop().unwrap();
}
