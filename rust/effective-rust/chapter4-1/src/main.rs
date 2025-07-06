use std::io::Error;

pub mod somemodule {
    #[derive(Debug, Default)]
    pub struct AStruct {
        count: i32,
        pub name: String,
    }
    impl AStruct {
        fn canonical_name(&self) -> String {
            self.name.to_lowercase()
        }
        pub fn id(&self) -> String {
            format!("{}-{}", self.canonical_name(), self.count)
        }
    }

    pub enum AnEnum {
        VariantOne,
        VariantTwo(u32),
        VariantThree { name: String, value: String },
    }

    pub trait DoSomething {
        fn do_something(&self) -> String;
    }
}

pub mod anothermodule {
    fn inaccessible_fn(x: i32) -> i32 {
        x + 3
    }
}

fn main() {
    #[cfg(myname = "a")]
    println!("cfg(myname = 'a') is set");
    #[cfg(myname = "b")]
    println!("cfg(myname = 'b') is set");

    #[derive(Debug)]
    pub struct ExposedStruct {
        pub data: Vec<u8>,
        #[cfg(feature = "schema")]
        pub schema: String,
    }

    let s = ExposedStruct {
        data: vec![1, 2, 3],
        #[cfg(feature = "schema")]
        schema: "example_schema".to_string(),
    };

    pub trait AsCbor: Sized {
        fn serialize(&self) -> Result<Vec<u8>, Error>;
        fn deserialize(data: &[u8]) -> Result<Self, Error>;
        #[cfg(feature = "schema")]
        fn cddl(&self) -> String;
    }
}
