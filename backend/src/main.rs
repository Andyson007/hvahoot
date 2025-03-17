use std::{convert::Infallible, io};

use backend::http::Header;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<Infallible> {
    let listener = TcpListener::bind("127.0.0.1:8444").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        handle_socket(socket).await;
    }
}

async fn handle_socket(mut stream: TcpStream) {
    let data = Header::new(&mut stream).await;

    println!("{data:#?}");

    let response = "HTTP/1.1 200 OK\r\n\r\nhello";
    if data.is_ok() {
        stream.write_all(response.as_bytes()).await.unwrap();
    }
}
