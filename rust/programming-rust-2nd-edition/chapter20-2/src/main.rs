use std::rc::Rc;

use anyhow::Result;
use async_std::io::prelude::*;
use async_std::{net, task};

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

fn cheapo_request_block(
    host: &str,
    port: u16,
    path: &str,
) -> impl Future<Output = std::io::Result<()>> + 'static {
    let host = host.to_string();
    let path = path.to_string();

    async move { Ok(()) }
}

async fn cheapo_owning_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    cheapo_request(&host, port, &path).await
}

pub async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {
            cheapo_request(&host, port, &path).await
        }));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

async fn reluctant() -> String {
    let return_value = {
        let string = Rc::new("ref-counted string".to_string());

        format!("{}", string)
    };

    net::TcpStream::connect(("localhost", 8087)).await.unwrap();

    return_value
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn some_fallible_thing() -> GenericResult<i32> {
    Ok(42)
}

async fn unfortunate() {
    match some_fallible_thing() {
        Err(error) => {
            eprintln!("Error: {}", error);
        }
        Ok(output) => {
            println!("Output: {}", output);

            net::TcpStream::connect(("localhost", 8087)).await.unwrap();
        }
    }
}

async fn verify_password(
    password: &str,
    hash: &str,
    key: &str,
) -> Result<bool, argonautica::Error> {
    let password = password.to_string();
    let hash = hash.to_string();
    let key = key.to_string();

    async_std::task::spawn_blocking(move || {
        argonautica::Verifier::default()
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(key)
            .verify()
    })
    .await
}

fn main() -> Result<()> {
    // let response = task::block_on(cheapo_request("example.com", 80, "/"))?;
    // println!("{}", response);

    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];
    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    // let serve_one = async {
    //     let listener = net::TcpListener::bind("localhost:8087").await?;
    //     let (mut socket, _addr) = listener.accept().await?;

    //     Ok(())
    // };

    let input = async_std::io::stdin();
    let future = async {
        let mut line = String::new();
        input.read_line(&mut line).await?;
        println!("Read line: {}", line);

        anyhow::Ok(())
    };

    task::spawn(reluctant());

    async_std::task::spawn(unfortunate());

    Ok(())
}
