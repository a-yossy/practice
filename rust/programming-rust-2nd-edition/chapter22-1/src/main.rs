use my_ascii::Ascii;

mod my_ascii {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii(Vec<u8>);

    impl Ascii {
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, NotAsciiError> {
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }

        /// # Safety
        pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
            Ascii(bytes)
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>);

    impl From<Ascii> for String {
        fn from(value: Ascii) -> Self {
            unsafe { String::from_utf8_unchecked(value.0) }
        }
    }
}

// fn very_trustworthy(shared: &i32) {
//     unsafe {
//         let mutable = shared as *const i32 as *mut i32;
//         *mutable = 20;
//     }
// }

pub unsafe trait Zeroable {}
unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for usize {}

fn zeroed_vector<T>(len: usize) -> Vec<T>
where
    T: Zeroable,
{
    let mut vec = Vec::with_capacity(len);
    unsafe {
        std::ptr::write_bytes(vec.as_mut_ptr(), 0, len);
        vec.set_len(len);
    }
    vec
}

struct HoldsRef<'a>(&'a mut i32);
unsafe impl<'a> Zeroable for HoldsRef<'a> {}

fn main() {
    let mut a: usize = 0;
    let ptr = &mut a as *mut usize;
    unsafe {
        *ptr.offset(3) = 0x7ffff72f484c;
    }

    let bytes: Vec<u8> = b"ASCII and ye shall receive".to_vec();
    let ascii: Ascii = Ascii::from_bytes(bytes).unwrap();
    let string = String::from(ascii);
    assert_eq!(string, "ASCII and ye shall receive");

    // let bytes = vec![0xf7, 0xbf, 0xbf, 0xbf];
    // let ascii = unsafe { Ascii::from_bytes_unchecked(bytes) };
    // let bogus: String = ascii.into();
    // assert_eq!(bogus.chars().next().unwrap() as u32, 0x1fffff);

    let v: Vec<usize> = zeroed_vector(100_000);
    assert!(v.iter().all(|&x| x == 0));

    let mut v: Vec<HoldsRef> = zeroed_vector(1);
    *v[0].0 = 1;
}
