use std::io;

use backend::http::Http;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8444").await?;

    loop {
        let (socket, x) = listener.accept().await?;
        handle_socket(socket).await;
    }
    Ok(())
}

async fn handle_socket(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let data = Http::new(&mut buf_reader).await;
    
    println!("{data:#?}");

    let response = "HTTP/1.1 200 OK\r\n\r\nhello";

    stream.write_all(response.as_bytes()).await.unwrap();
}
