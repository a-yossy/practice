use core::net;
use std::{
    io::{Read, Write},
    net::TcpStream,
    pin::Pin,
    task::Context,
    thread,
};

use anyhow::Result;
use async_std::stream::StreamExt;

fn chat_group_table() -> String {
    String::new()
}

async fn async_chat() -> Result<()> {
    let address = "127.0.0.1:8080";
    let listener = async_std::net::TcpListener::bind(address).await?;
    let mut new_connections = listener.incoming();
    while let Some(socket_result) = new_connections.next().await {
        let _socket = socket_result?;
        let _groups = chat_group_table().clone();
        async_std::task::spawn(async {
            println!("New connection");
        });
    }

    Ok(())
}

fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = TcpStream::connect((host, port))?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

enum Poll<T> {
    Ready(T),
    Pending,
}

trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, ct: &mut Context<'_>) -> Poll<Self::Output>;
}

// fn read_to_string<'a>(
//     &'a mut self,
//     buf: &'a mut String,
// ) -> impl Future<Output = Result<usize>> + 'a {
//     let mut bytes = vec![0; 1024];
//     let n = self.read(&mut bytes)?;
//     buf.push_str(std::str::from_utf8(&bytes[..n])?);
//     Ok(n)
// }

fn main() -> Result<()> {
    let address = "127.0.0.1:8080";
    let listener = std::net::TcpListener::bind(address)?;
    for socket_result in listener.incoming() {
        let _socket = socket_result?;
        let _groups = chat_group_table().clone();
        thread::spawn(|| {
            println!("New connection");
        });
    }

    Ok(())
}
