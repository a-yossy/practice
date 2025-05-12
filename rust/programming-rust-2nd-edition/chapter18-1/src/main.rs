use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::io::{BufReader, prelude::*};
use std::net::TcpListener;
#[cfg(unix)]
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread::spawn;

use anyhow::Result;

#[cfg(not(unix))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst: Q) -> std::io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("can't copy symbolic link: {}", src.as_ref().display()),
    ))
}

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {}", addr);
    loop {
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from {}", addr);

        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            io::copy(&mut stream, &mut write_stream).expect("error in client thread: ");
            println!("connection closed");
        });
    }
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?,
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }

    Ok(())
}

fn http_get_main(url: &str) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        Err(format!("{}", response.status()))?;
    }

    let stdout = io::stdout();
    io::copy(&mut response, &mut stdout.lock())?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: http-get URL");
        return Ok(());
    }

    if let Err(err) = http_get_main(&args[1])  {
        eprintln!("error: {}", err);
    }

    echo_main("127.0.0.1:17007").expect("error: ");

    let target = "hello";
    let stdin = io::stdin();
    grep(&target, stdin.lock()).unwrap();

    let f = File::open("hello.txt").unwrap();
    grep(&target, BufReader::new(f)).unwrap();

    let result = grep_main();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    let lines = BufReader::new(File::open("hello.txt").unwrap())
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .unwrap();

    writeln!(io::stderr(), "error: world not helloable")?;
    let mut byte_vec = Vec::new();
    writeln!(
        &mut byte_vec,
        "The greatest common divisor of {:?} is {}",
        1, 2
    )?;

    let log = OpenOptions::new().append(true).open("server.log")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("new_file.txt")?;

    let mut child = Command::new("grep")
        .arg("-e")
        .arg("a.*e.*i.*o.*u")
        .stdin(Stdio::piped())
        .spawn()?;
    let mut to_child = child.stdin.take().unwrap();
    let my_words = vec!["a", "e", "i", "o", "u", "hello", "world"];
    for word in my_words {
        writeln!(to_child, "{}", word)?;
    }
    drop(to_child);
    child.wait()?;

    type RoomId = String;
    type RoomExits = Vec<(char, RoomId)>;
    type RoomMap = HashMap<RoomId, RoomExits>;
    let mut map = RoomMap::new();
    map.insert("room1".to_string(), vec![('n', "room2".to_string())]);
    map.insert("room2".to_string(), vec![('s', "room1".to_string())]);
    serde_json::to_writer(&mut std::io::stdout(), &map)?;

    fn swizzle_file<P>(path_arg: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path_arg.as_ref();

        Ok(())
    }

    let home_dir = Path::new("/home/user");

    fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
        if src_type.is_file() {
            fs::copy(src, dst)?;
        } else if src_type.is_dir() {
            copy_dir_to(src, dst)?;
        } else if src_type.is_symlink() {
            let target = src.read_link()?;
            symlink(target, dst)?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("don't know how to copy: {}", src.display()),
            ));
        }

        Ok(())
    }

    fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
        if !dst.is_dir() {
            fs::create_dir(dst)?;
        }

        for entry_result in src.read_dir()? {
            let entry = entry_result?;
            let file_type = entry.file_type()?;
            copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
        }

        Ok(())
    }

    Ok(())
}
