macro_rules! let_foo {
    ($x:expr) => {
        let foo = $x;
    };
}

macro_rules! please_set {
    ($i:ident, $x:expr) => {
        $i = $x;
    };
}

macro_rules! name_as_debug {
    ($t:ty) => {
        impl ::core::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::write!(f, ::core::stringify!($t))
            }
        }
    };
}

fn main() {
    let foo = 0;
    let_foo!(42);
    println!("Hello, world! {}", foo);

    let mut x = 1;
    please_set!(x, x + 1);
    println!("x is now: {}", x);
}
