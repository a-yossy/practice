#[repr(C)]
pub struct FfiStruct {
    pub byte: u8,
    pub integer: u32,
}

extern "C" {
    pub fn new_struct(v: u32) -> *mut FfiStruct;
    pub fn free_struct(s: *mut FfiStruct);
}

struct FfiWrapper {
    inner: *mut FfiStruct,
}

impl Drop for FfiWrapper {
    fn drop(&mut self) {
        unsafe {
            free_struct(self.inner);
        }
    }
}

type Error = String;

impl FfiWrapper {
    pub fn new(val: u32) -> Result<Self, Error> {
        let p: *mut FfiStruct = unsafe { new_struct(val) };
        if p.is_null() {
            Err("Failed to create FfiStruct".to_string())
        } else {
            Ok(Self { inner: p })
        }
    }

    pub fn set_byte(&mut self, b: u8) {
        let r: &mut FfiStruct = unsafe { &mut *self.inner };
        r.byte = b;
    }
}

impl AsMut<FfiStruct> for FfiWrapper {
    fn as_mut(&mut self) -> &mut FfiStruct {
        unsafe { &mut *self.inner }
    }
}

#[no_mangle]
pub extern "C" fn add_contents_safer(p: *const FfiStruct) -> u32 {
    let s = match unsafe { p.as_ref() } {
        Some(r) => r,
        None => return 0,
    };
    s.byte as u32 + s.integer
}

impl FfiStruct {
    pub fn new(v: u32) -> Self {
        FfiStruct {
            byte: 0,
            integer: v,
        }
    }
}

#[no_mangle]
pub extern "C" fn new_struct(v: u32) -> *mut FfiStruct {
    let mut s = FfiStruct::new(v);
    &mut s
}

#[no_mangle]
pub extern "C" fn new_struct_heap(v: u32) -> *mut FfiStruct {
    let s = Box::new(FfiStruct::new(v));
    &mut *s
}

#[no_mangle]
pub extern "C" fn new_struct_raw(v: u32) -> *mut FfiStruct {
    let s = FfiStruct::new(v);
    let b = Box::new(s);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn free_struct_raw(s: *mut FfiStruct) {
    if s.is_null() {
        return;
    }
    let _b = unsafe { Box::from_raw(s) };
}

fn main() {
    println!("Hello, world!");
}
