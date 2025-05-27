union FloatOrInt {
    f: f32,
    i: i32,
}

union SmallOrLarge {
    s: bool,
    l: u64,
}

#[repr(C)]
union SignExtractor {
    value: i64,
    bytes: [u8; 8],
}

fn sign(int: i64) -> bool {
    let se = SignExtractor { value: int };
    println!("{:b} ({:?})", unsafe { se.value }, unsafe { se.bytes });
    unsafe { se.bytes[7] >= 0b10000000 }
}

fn main() {
    let mut one = FloatOrInt { i: 1 };
    assert_eq!(unsafe { one.i }, 0x00_00_00_01);
    one.f = 1.0;
    assert_eq!(unsafe { one.i }, 0x3F_80_00_00);

    let u = SmallOrLarge { l: 1337 };
    println!("{}", unsafe { u.l });
    unsafe {
        match u {
            SmallOrLarge { s: true } => {
                println!("boolean true");
            }
            SmallOrLarge { l: 2 } => {
                println!("integer 2");
            }
            _ => {
                println!("something else");
            }
        }
    }
    unsafe {
        match u {
            SmallOrLarge { s } => {
                println!("boolean: {}", s);
            }
            SmallOrLarge { l } => {
                println!("integer: {}", l);
            }
        }
    }

    let float = FloatOrInt { f: 31337.0 };
    println!("{:b}", unsafe { float.i });

    assert_eq!(sign(-1), true);
    assert_eq!(sign(1), false);
    assert_eq!(sign(i64::MAX), false);
    assert_eq!(sign(i64::MIN), true);
}
