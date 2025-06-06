use std::{any::Any, fmt, io, thread};

pub enum CopyError {
    In(std::io::Error),
    Out(std::io::Error),
}

#[derive(Debug)]
enum InternalImageError {
    InvalidHeaderSize,
    DecompressionFailed,
}

impl fmt::Display for InternalImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalImageError::InvalidHeaderSize => write!(f, "Invalid header size"),
            InternalImageError::DecompressionFailed => write!(f, "Decompression failed"),
        }
    }
}

impl std::error::Error for InternalImageError {}

fn decode_image_public(data: &[u8]) -> Result<(), Box<dyn std::error::Error + 'static>> {
    if data.len() < 10 {
        return Err(Box::new(InternalImageError::InvalidHeaderSize));
    }

    // Simulate decompression failure
    if data[0] == 0xFF {
        return Err(Box::new(InternalImageError::DecompressionFailed));
    }

    Ok(())
}

fn process_file(path: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let _file = std::fs::File::open(path)?;
    Ok(())
}

// type Result<T> = Result<T, Box<dyn Any + Send + 'static>>;

// fn run_server() -> Result<!, String> {
//     println!("Server is running...");
//     loop {
//         println!("Waiting for connections...");
//     }
// }

fn main() {
    let result = process_file("example.txt");
    match result {
        Ok(_) => println!("File processed successfully."),
        Err(e) => {
            eprintln!("Error processing file: {}", e);
            if let Some(io_err) = e.downcast_ref::<io::Error>() {
                if io_err.kind() == io::ErrorKind::NotFound {
                    eprintln!("File not found: {}", io_err);
                } else {
                    eprintln!("IO error: {}", io_err);
                }
            }
        }
    }

    let handle = thread::spawn(|| {
        panic!("Simulated panic in thread");
    });

    match handle.join() {
        Ok(_) => println!("Thread completed successfully."),
        Err(e) => {
            if let Some(panic_msg) = e.downcast_ref::<&'static str>() {
                eprintln!("Thread panicked with message: {}", panic_msg);
            } else {
                eprintln!("Thread panicked with an unknown error.");
            }
        }
    }
}
