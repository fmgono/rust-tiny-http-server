use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Connection established!");

        socket
            .write_all(b"HTTP/1.1 200 OK\r\n\r\nHello from Rust!")
            .await
            .unwrap();
    }

    Ok(())
}
