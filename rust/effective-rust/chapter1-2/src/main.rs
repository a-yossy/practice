use core::fmt;
use std::{cell::RefCell, collections::HashSet, error::Error, io::BufRead, rc::Rc, vec};

use anyhow::Result;

fn main() -> Result<()> {
    #[derive(Debug)]
    pub enum MyError {
        Io(std::io::Error),
        Utf8(std::string::FromUtf8Error),
        General(String),
    }
    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyError::Io(err) => write!(f, "IO error: {}", err),
                MyError::Utf8(err) => write!(f, "UTF-8 error: {}", err),
                MyError::General(msg) => write!(f, "Error: {}", msg),
            }
        }
    }
    impl std::error::Error for MyError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                MyError::Io(err) => Some(err),
                MyError::Utf8(err) => Some(err),
                MyError::General(_) => None,
            }
        }
    }
    impl From<std::io::Error> for MyError {
        fn from(value: std::io::Error) -> Self {
            Self::Io(value)
        }
    }
    impl From<std::string::FromUtf8Error> for MyError {
        fn from(value: std::string::FromUtf8Error) -> Self {
            Self::Utf8(value)
        }
    }
    const MAX_LEN: usize = 1024;
    pub fn first_line(filename: &str) -> Result<String, MyError> {
        let file = std::fs::File::open(filename)?;
        let mut reader = std::io::BufReader::new(file);
        let mut buf = vec![];
        let len = reader.read_until(b'\n', &mut buf)?;
        let result = String::from_utf8(buf)?;
        if result.len() > MAX_LEN {
            return Err(MyError::General(format!(
                "Input too long: {}",
                result.len()
            )));
        }
        Ok(result)
    }

    #[derive(Debug)]
    pub enum WrappedError {
        Wrapped(Box<dyn std::error::Error>),
        General(String),
    }
    impl std::fmt::Display for WrappedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Wrapped(e) => write!(f, "Inner error: {}", e),
                Self::General(s) => write!(f, "{}", s),
            }
        }
    }
    impl Error for WrappedError {}
    // impl<E: 'static + Error> From<E> for WrappedError {
    //     fn from(value: E) -> Self {
    //         Self::Wrapped(Box::new(value))
    //     }
    // }

    let x: u32 = 2;
    // let y: u64 = x;

    // impl<T, U> Into<U> for T
    // where
    //     U: From<T>,
    // {
    //     fn into(self) -> U {
    //         U::from(self)
    //     }
    // }

    // impl<T> From<T> for T {
    //     fn from(value: T) -> Self {
    //         value
    //     }
    // }

    #[derive(Clone, Copy, Debug)]
    pub struct IanaAllocated(pub u64);
    impl From<u64> for IanaAllocated {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    pub fn is_iana_reserved<T>(s: T) -> bool
    where
        T: Into<IanaAllocated>,
    {
        let s = s.into();
        s.0 == 0 || s.0 == 65535
    }
    let s = IanaAllocated(1);
    println!("{:?} reserved? {}", s, is_iana_reserved(s));
    println!("{}", is_iana_reserved(11));

    let x: u32 = 9;
    let y = x as u64;
    let z: u64 = x.into();
    let y = x as u16;

    pub struct PoundForceSeconds(pub f64);
    pub fn thruster_impulse(direction: i64) -> PoundForceSeconds {
        return PoundForceSeconds(direction as f64 * 0.5);
    }
    pub struct NewtonSeconds(pub f64);
    pub fn update_trajectory(force: NewtonSeconds) {}
    impl From<PoundForceSeconds> for NewtonSeconds {
        fn from(value: PoundForceSeconds) -> Self {
            Self(value.0 * 4.4482216152605)
        }
    }
    let thruster_force = thruster_impulse(10);
    let new_direction = update_trajectory(thruster_force.into());

    // impl std::fmt::Display for rand::rngs::StdRng {
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     write!(f, "StdRng")
    // }
    struct MyRng(rand::rngs::StdRng);
    impl fmt::Display for MyRng {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MyRng")
        }
    }

    #[derive(Debug, Clone)]
    pub struct PhoneNumberE164(pub String);
    #[derive(Debug, Clone)]
    pub struct Date;

    #[derive(Debug)]
    pub struct Details {
        pub given_name: String,
        pub preferred_name: Option<String>,
        pub middle_name: Option<String>,
        pub family_name: String,
        pub mobile_phone: Option<PhoneNumberE164>,
        pub date_of_birth: Date,
        pub last_seen: Option<Date>,
    }

    let dizzy = Details {
        given_name: "Dizzy".to_string(),
        preferred_name: Some("Diz".to_string()),
        middle_name: Some("D".to_string()),
        family_name: "Dizzy".to_string(),
        mobile_phone: Some(PhoneNumberE164("+1-555-1234".to_string())),
        date_of_birth: Date,
        last_seen: None,
    };

    pub struct DetailsBuilder(Details);
    impl DetailsBuilder {
        pub fn new(given_name: &str, family_name: &str, date_of_birth: Date) -> Self {
            Self(Details {
                given_name: given_name.to_owned(),
                preferred_name: None,
                middle_name: None,
                family_name: family_name.to_owned(),
                mobile_phone: None,
                date_of_birth,
                last_seen: None,
            })
        }

        pub fn preferred_name(mut self, preferred_name: &str) -> Self {
            self.0.preferred_name = Some(preferred_name.to_owned());
            self
        }

        pub fn middle_name(mut self, middle_name: &str) -> Self {
            self.0.middle_name = Some(middle_name.to_owned());
            self
        }

        pub fn just_seen(mut self) -> Self {
            self.0.last_seen = Some(Date);
            self
        }

        pub fn just_seen2(&mut self) -> &mut Self {
            self.0.last_seen = Some(Date);
            self
        }

        pub fn build(self) -> Details {
            self.0
        }

        pub fn build2(&self) -> Details {
            todo!()
        }
    }

    let also_bob = DetailsBuilder::new("Bob", "Builder", Date)
        .preferred_name("Bob")
        .middle_name("B")
        .just_seen()
        .build();

    let mut builder = DetailsBuilder::new("Alice", "Smith", Date);
    builder = builder.preferred_name("Ali");
    let bob = builder.build();

    let smithy = DetailsBuilder::new("Smithy", "Smith", Date);
    // let clones = vec![smithy.build(), smithy.build(), smithy.build()];

    let mut builder = DetailsBuilder::new("Alice", "Smith", Date);
    builder.just_seen2();
    let alice = builder.build();

    let mut builder = DetailsBuilder::new("Alice", "Smith", Date);
    builder.just_seen2();
    let alice = builder.build();

    // fn dangle() -> &'static i64 {
    //     let x: i64 = 32;
    //     &x
    // }

    pub struct Point {
        pub x: u32,
        pub y: u32,
    }
    let pt = Point { x: 1, y: 2 };
    let x = 0u64;
    let ref_x = &x;
    let ref_pt = &pt;
    let box_pt = Box::new(Point { x: 3, y: 4 });

    fn show(pt: &Point) {
        println!("({}, {})", pt.x, pt.y);
    }
    show(ref_pt);
    show(&box_pt);

    let array: [u64; 5] = [0, 1, 2, 3, 4];
    let slice = &array[1..3];

    let mut vector = Vec::<u64>::with_capacity(8);
    for i in 0..5 {
        vector.push(i);
    }
    let vslice = &vector[1..3];

    trait Calculate {
        fn add(&self, l: u64, r: u64) -> u64;
        fn mul(&self, l: u64, r: u64) -> u64;
    }
    struct Modulo(pub u64);
    impl Calculate for Modulo {
        fn add(&self, l: u64, r: u64) -> u64 {
            (l + r) % self.0
        }
        fn mul(&self, l: u64, r: u64) -> u64 {
            (l * r) % self.0
        }
    }
    let mod3 = Modulo(3);
    let tobj: &dyn Calculate = &mod3;
    let result = tobj.add(2, 2);
    assert_eq!(result, 1);

    fn add_four<T: std::borrow::Borrow<i32>>(v: T) -> i32 {
        v.borrow() + 4
    }
    assert_eq!(add_four(&2), 6);
    assert_eq!(add_four(2), 6);

    let rc1: Rc<u64> = Rc::new(42);
    let rc2 = rc1.clone();
    let wk = Rc::downgrade(&rc1);

    let rc: RefCell<u64> = RefCell::new(42);
    let b1 = rc.borrow();
    let b2 = rc.borrow();

    let values: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7];
    let mut even_sum_squares = 0;
    let mut even_count = 0;
    for i in 1..values.len() {
        if values[i] % 2 != 0 {
            continue;
        }
        even_sum_squares += values[i] * values[i];
        even_count += 1;
        if even_count == 5 {
            break;
        }
    }

    let even_sum_squares: u64 = values
        .iter()
        .filter(|x| *x % 2 == 0)
        .take(5)
        .map(|x| x * x)
        .sum();

    let collection = vec![1];
    for item in collection.clone() {}
    let mut iter = collection.clone().into_iter();
    loop {
        let item = match iter.next() {
            Some(item) => item,
            None => break,
        };
    }
    let mut iter = collection.into_iter();
    while let Some(item) = iter.next() {}

    #[derive(Debug)]
    struct Thing(pub u64);
    let collection = vec![Thing(1), Thing(2), Thing(3)];
    for item in &collection {
        println!("Consumed item: {item:?}");
    }
    let mut iter = (&collection).into_iter();
    while let Some(item) = iter.next() {
        println!("{}", item.0);
    }
    println!("Collection = {collection:?}");

    let result: u64 = (&collection).into_iter().map(|thing| thing.0).sum();
    let result: u64 = collection.iter().map(|thing| thing.0).sum();

    let mut even_sum_squares = 0;
    for value in values.iter().filter(|x| *x % 2 == 0).take(5) {
        even_sum_squares += value * value;
    }

    let mut even_sum_squares = 0;
    values
        .iter()
        .filter(|x| *x % 2 == 0)
        .take(5)
        .for_each(|x| even_sum_squares += x * x);

    let myvec: Vec<i32> = (0..10).into_iter().filter(|x| x % 2 == 0).collect();
    let h: HashSet<i32> = (0..10).into_iter().filter(|x| x % 2 == 0).collect();

    let inputs: Vec<i64> = vec![1, 2, 3, 4, 5];
    let result: Vec<u8> = inputs
        .into_iter()
        .map(|v| <u8>::try_from(v).unwrap())
        .collect();

    let inputs: Vec<i64> = vec![1, 2, 3, 4, 5, 512];
    let result: Vec<Result<u8, _>> = inputs
        .clone()
        .into_iter()
        .map(|v| <u8>::try_from(v))
        .collect();
    let result: Vec<u8> = inputs
        .into_iter()
        .map(|v| <u8>::try_from(v))
        .collect::<Result<Vec<_>, _>>()?;

    let even_sum_squares: u64 = values
        .iter()
        .filter(|x| *x % 2 == 0)
        .take(5)
        .map(|x| x * x)
        .sum();

    Ok(())
}
