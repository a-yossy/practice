use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

fn main() {
    fn divide(a: i64, b: i64) -> i64 {
        if b == 0 {
            panic!("Division by zero is not allowed");
        }
        a / b
    }

    // let result = divide(0, 0);

    fn divide_recover(a: i64, b: i64, default: i64) -> i64 {
        let result = std::panic::catch_unwind(|| divide(a, b));
        match result {
            Ok(value) => value,
            Err(_) => default,
        }
    }

    let result = divide_recover(0, 0, 42);
    println!("Result: {}", result);

    fn tname<T: ?Sized>(_v: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    fn type_id<T: 'static + ?Sized>(_v: &T) -> TypeId {
        TypeId::of::<T>()
    }

    let x = 42u32;
    let y = vec![3, 4, 2];
    println!("x: {}. {}", tname(&x), x);
    println!("y: {}. {:?}", tname(&y), y);

    println!("x had {:?}", type_id(&x));
    println!("y had {:?}", type_id(&y));

    let x_any: Box<dyn Any> = Box::new(42u64);
    let y_any: Box<dyn Any> = Box::new(vec![3, 4, 2]);

    trait Draw: Debug {
        fn bounds(&self) -> (i32, i32);
    }

    trait Shape: Draw {
        fn render_in(&self, bounds: (i32, i32));
        fn render(&self) {
            self.render_in(self.bounds());
        }
    }

    #[derive(Debug)]
    struct Square {
        width: i32,
        height: i32,
    }

    impl Square {
        fn new(width: i32, height: i32) -> Self {
            Self { width, height }
        }
    }
    impl Draw for Square {
        fn bounds(&self) -> (i32, i32) {
            (self.width, self.height)
        }
    }
    impl Shape for Square {
        fn render_in(&self, bounds: (i32, i32)) {
            println!(
                "Rendering square with width {} and height {} in bounds {:?}",
                self.width, self.height, bounds
            );
        }
    }

    let square = Square::new(10, 20);
    let draw: &dyn Draw = &square;
    let shape: &dyn Shape = &square;

    #[derive(Debug, Clone)]
    pub struct Tlv {
        pub type_code: u8,
        pub value: Vec<u8>,
    }
    pub type Error = &'static str;
    pub fn get_next_tlv(input: &[u8]) -> Result<(Tlv, &[u8]), Error> {
        if input.len() < 2 {
            return Err("Input too short to contain TLV");
        }

        let type_code = input[0];
        let len = input[1] as usize;
        if 2 + len > input.len() {
            return Err("Input too short to contain TLV value");
        }
        let tlv = Tlv {
            type_code,
            value: input[2..2 + len].to_vec(),
        };
        Ok((tlv, &input[2 + len..]))
    }

    #[derive(Debug, Default)]
    pub struct NetworkServer {
        max_size: Option<Tlv>,
    }
    const SET_MAX_SIZE: u8 = 0x01;
    impl NetworkServer {
        pub fn process(&mut self, mut data: &[u8]) -> Result<(), Error> {
            while !data.is_empty() {
                let (tlv, rest) = get_next_tlv(data)?;
                match tlv.type_code {
                    SET_MAX_SIZE => {
                        self.max_size = Some(tlv);
                    }
                    _ => return Err("Unknown TLV type code"),
                }
                data = rest;
            }
            Ok(())
        }
        pub fn done(&self) -> bool {
            self.max_size.is_some()
        }
    }
    fn read_data_into_buffer(buffer: &mut [u8]) {
        let data = b"\x01\x03abc\x01\x02xy";
        buffer[..data.len()].copy_from_slice(data);
    }
    let mut perma_buffer = [0u8; 256];
    let mut server = NetworkServer::default();
    while !server.done() {
        read_data_into_buffer(&mut perma_buffer);
        if let Err(e) = server.process(&perma_buffer) {
            eprintln!("Error processing data: {}", e);
        }
    }
}
