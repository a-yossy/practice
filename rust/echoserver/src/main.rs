use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(address)?; // [1]
    loop {
        let (mut stream, _) = listener.accept()?; // [2]
        thread::spawn(move || {
            // [3]
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap(); // [4]
                if nbytes == 0 {
                    // [6]
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap(); // [5]
            }
        });
    }
}
