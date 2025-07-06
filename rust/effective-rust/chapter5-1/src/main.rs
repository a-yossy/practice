// fn before() {
//     println!("[before] square {} is {}", 2, square!(2));
// }

use serde::Deserialize;

macro_rules! square {
    { $e:expr } => { $e * $e }
}

fn after() {
    println!("[after] square {} is {}", 2, square!(2));
}

mod submod {
    #[macro_export]
    macro_rules! cube {
        { $e:expr } => { $e * $e * $e }
    }
}

mod user {
    pub fn use_macro() {
        let cubed = crate::cube!(3);
        println!("[user] cube {} is {}", 3, cubed);
    }
}

// macro_rules! increment_x {
//     {} => { x +=1; };
// }

macro_rules! inc_item {
    { $x:ident } => { $x.contents += 1; };
}

macro_rules! check_successful {
    { $e:expr } => {
        if $e.group() != Group::Successful {
            return Err(MyError("HTTP operation failed"));
        }
    };
}

macro_rules! check_success {
    { $e:expr } => {
        match $e.group() {
            Group::Successful => Ok(()),
            _ => Err(MyError("HTTP operation failed")),
        }
    };
}

macro_rules! square_once {
    { $e:expr } => {
        {
            let x = $e;
            x * x
        }
    };
}

macro_rules! my_log {
    { $($arg:tt)+ } => {
        eprintln!("{}:{}:{}", file!(), line!(), format_args!($($arg)+));
    };
}

fn generate_value() -> String {
    "Hello, world!".to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Group {
    Informational,
    Successful,
    Redirection,
    ClientError,
    ServerError,
}

macro_rules! http_codes {
    { $($name:ident => ($val:literal, $group:ident, $text: literal),)+ } => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
        enum Status {
            $($name = $val,)+
        }
        impl Status {
            fn group(&self) -> Group {
                match self {
                    $(Self::$name => Group::$group,)+
                }
            }
            fn text(&self) -> &'static str {
                match self {
                    $(Self::$name => $text,)+
                }
            }
        }
        impl core::convert::TryFrom<i32> for Status {
        type Error = ();
        fn try_from(v: i32) -> Result<Self, Self::Error> {
            match v {
                $($val => Ok(Self::$name),)+
                _ => Err(()),
                }
            }
        }
    }
}

macro_rules! log_failure {
    { $e:expr } => {
        {
            let result = $e;
            if let Err(err) = &result {
                eprintln!("{}:{}:operation '{}' failed: {:?}", file!(), line!(), stringify!($e), err);
            }
            result
        }
    };
}

#[derive(Debug, Deserialize)]
struct MyData {
    #[serde(default = "generate_value")]
    value: String,
}

enum Multi {
    Byte(u8),
    Int(i32),
    Str(String),
}

#[macro_export]
macro_rules! values_of_type {
    { $values:expr, $variant:ident } => {
        {
            let mut result = Vec::new();
            for val in $values {
                if let Multi::$variant(v) = val {
                    result.push(v.clone());
                }
            }
            result
        }
    };
}

fn main() {
    #[derive(Debug)]
    struct Item {
        contents: i32,
    }
    let mut x = Item { contents: 42 };
    inc_item!(x);
    println!("x is {x:?}");

    let values = vec![
        Multi::Byte(1),
        Multi::Int(1000),
        Multi::Str("a string".to_string()),
        Multi::Byte(2),
    ];
    let ints = values_of_type!(&values, Int);
    println!("ints are {:?}", ints);

    let bytes = values_of_type!(&values, Byte);
    println!("bytes are {:?}", bytes);

    http_codes! {
        Continue           => (100, Informational, "Continue"),
        SwitchingProtocols => (101, Informational, "Switching Protocols"),
        // ...
        Ok                 => (200, Successful, "Ok"),
        Created            => (201, Successful, "Created"),
    }

    use std::convert::TryInto;
    log_failure!(<i32 as TryInto<u8>>::try_into(100));
    let x: Result<u8, _> = log_failure!(512.try_into());
}
