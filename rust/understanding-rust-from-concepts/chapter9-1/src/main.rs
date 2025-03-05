#[link(name = "z")]
extern "C" {
    fn compress(dest: *mut u8, destlen: *mut usize, src: *const u8, srclen: usize) -> i32;
    fn uncompress(dest: *mut u8, destlen: *mut usize, src: *const u8, srclen: usize) -> i32;
}

const BUFSIZE: usize = 1024;

fn main() {
    let mut compressed_buf = vec![0_u8; BUFSIZE];
    let mut uncompressed_buf = vec![0_u8; BUFSIZE];

    let orgdata = "Hello, Hello, Hello, world!!!";
    let orgdata_size = orgdata.len();

    let mut destlen_comp = BUFSIZE;
    let mut destlen_uncomp = BUFSIZE;

    println!("orgdata: {}", orgdata);
    unsafe {
        let ret_comp = compress(
            compressed_buf.as_mut_ptr(),
            &mut destlen_comp as *mut usize,
            orgdata.as_ptr(),
            orgdata_size,
        );
        println!("ret_comp: {}", ret_comp);
    }
    println!{"destlen_comp: {}", destlen_comp};
    println!();

    unsafe {
        let ret_uncomp = uncompress(
            uncompressed_buf.as_mut_ptr(),
            &mut destlen_uncomp as *mut usize,
            compressed_buf.as_ptr(),
            destlen_comp,
        );
        println!("ret_uncomp: {}", ret_uncomp);
    }
    println!("destlen_uncomp: {}", destlen_uncomp);
    println!(
        "uncompressed: {}",
        String::from_utf8(uncompressed_buf[..destlen_uncomp].to_vec()).unwrap()
    )
}
