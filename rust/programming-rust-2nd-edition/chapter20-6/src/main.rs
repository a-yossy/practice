use std::pin::Pin;

use async_std::io::prelude::*;
use async_std::{io, net};

async fn fetch_string(address: &str) -> io::Result<String> {
    let mut socket = net::TcpStream::connect(address).await?;
    let mut buf = String::new();
    socket.read_to_string(&mut buf).await?;
    Ok(buf)
}

fn main() {
    let response = fetch_string("localhost:6502");
    let new_variable = response;

    let mut string = "Pinned?".to_string();
    let mut pinned: Pin<&mut String> = Pin::new(&mut string);

    pinned.push_str(" Not");
    Pin::into_inner(pinned).push_str(" so much.");

    let new_home = string;
    assert_eq!(new_home, "Pinned? Not so much.");
}
