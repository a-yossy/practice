use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    ffi::c_int,
    fmt::Display,
    hash::Hash,
    net::Ipv4Addr,
    ops::{Deref, DerefMut},
    path::Path,
};

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!("(AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

struct FileDesc {
    fd: c_int,
}

// impl Drop for FileDesc {
//     fn drop(&mut self) {
//         let _ = unsafe { libc::close(self.fd) };
//     }
// }

struct S<T: ?Sized> {
    b: Box<T>,
}

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

// trait Deref {
//     type Target: ?Sized;
//     fn deref(&self) -> &Self::Target;
// }

// trait DerefMut: Deref {
//     fn deref_mut(&mut self) -> &mut Self::Target;
// }

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

// trait Default {
//     fn default() -> Self;
// }

// impl Default for String {
//     fn default() -> Self {
//         String::new()
//     }
// }

trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}

// fn open<P: AsRef<Path>>(path: P) -> Result<File>

impl<'a, T, U> AsRef<U> for &'a T
where
    T: AsRef<U>,
    T: ?Sized,
    U: ?Sized,
{
    fn as_ref(&self) -> &U {
        (*self).as_ref()
    }
}

trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

// impl<K, V> HashMap<K, V>
// where
//     K: Eq + Hash,
// {
//     fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
//     where
//         K: Borrow<Q>,
//         Q: Eq + Hash,
//     {
//     }
// }

// trait Into<T>: Sized {
//     fn into(self) -> T;
// }

trait From<T>: Sized {
    fn from(other: T) -> Self;
}

fn ping<A>(address: A) -> std::io::Result<bool>
where
    A: Into<Ipv4Addr>,
{
    let ipv4_address = address.into();

    Ok(true)
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn parse_i32_bytes(b: &[u8]) -> GenericResult<i32> {
    Ok(std::str::from_utf8(b)?.parse::<i32>()?)
}

// impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a> {
//     fn from(err: E) -> Self {
//         Box::new(err)
//     }
// }

// pub trait TryFrom<T>: Sized {
//     type Error;
//     fn try_from(value: T) -> Result<Self, Self::Error>;
// }

// pub trait TryInto<T>: Sized {
//     type Error;
//     fn try_into(self) -> Result<T, Self::Error>;
// }

// trait ToOwned {
//     type Owned: Borrow<Self>;
//     fn to_owned(&self) -> Self::Owned;
// }

// enum Cow<'a, B: ?Sized + 'a>
// where
//     B: ToOwned,
// {
//     Borrowed(&'a B),
//     Owned(<B as ToOwned>::Owned),
// }

enum Error {
    OutOfMemory,
    StackOverflow,
    MachineOnFire,
    Unfathomable,
    FileNotFound(std::path::PathBuf),
}

fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        Error::OutOfMemory => "out of memory".into(),
        Error::StackOverflow => "stack overflow".into(),
        Error::MachineOnFire => "machine on fire".into(),
        Error::Unfathomable => "machine bewildered".into(),
        Error::FileNotFound(ref path) => format!("file not found: {}", path.display()).into(),
    }
}

fn main() {
    {
        let mut a = Appellation {
            name: String::from("Alice"),
            nicknames: vec![String::from("Ally"), String::from("Lice")],
        };
        println!("before assignment");
        a = Appellation {
            name: String::from("Bob"),
            nicknames: vec![String::from("Bobby"), String::from("Bobster")],
        };
        println!("at end of block");
    }

    let p;
    {
        let q = Appellation {
            name: String::from("Charlie"),
            nicknames: vec![String::from("Chuck"), String::from("Char")],
        };
        if true {
            p = q;
        }
    }
    println!("Sproing! What was that?");

    let boxed_lunch = RcBox {
        ref_count: 1,
        value: String::from("Tuna salad"),
    };

    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;
    fn display(boxed: &RcBox<dyn Display>) {
        println!("{}", &boxed.value);
    }
    display(&boxed_lunch);

    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };
    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'a';
    assert_eq!(s.elements, ['x', 'y', 'a']);

    let s = Selector {
        elements: vec!["good", "bad", "ugly"],
        current: 2,
    };
    fn show_it(thing: &str) {
        println!("{}", thing);
    }
    show_it(&s);

    fn show_it_generic<T: Display>(thing: T) {
        println!("{}", thing);
    }
    show_it_generic(*s);

    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & (n - 1) == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    let (upper, lower): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");

    println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141)));
    println!("{:?}", ping([66, 66, 66, 66]));
    println!("{:?}", ping(0xd076eb94_u32));

    let addr1 = Ipv4Addr::from([66, 146, 219, 98]);
    let addr2 = Ipv4Addr::from(0xd076eb94_u32);

    let text = "Beautiful Soup".to_string();
    let bytes: Vec<u8> = text.into();

    let huge = 10_i32;
    let smaller: i32 = huge
        .try_into()
        .unwrap_or_else(|_| if huge >= 0 { 1 } else { -1 });

    let error = Error::MachineOnFire;
    println!("error: {}", describe(&error));
    let mut log: Vec<String> = Vec::new();
    log.push(describe(&error).into_owned());
}
