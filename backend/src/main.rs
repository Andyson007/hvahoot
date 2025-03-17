use std::io;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
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

async fn handle_socket(mut socket: TcpStream) {
    let buf_reader = BufReader::new(&mut socket);
    let mut iter = buf_reader.lines();
    let http_request = {
        let mut ret = vec![];
        while let Ok(Some(next)) = iter.next_line().await {
            if next.is_empty() {
                break;
            }
            ret.push(next);
        }
        ret
    };
    println!("{http_request:#?}");
}
