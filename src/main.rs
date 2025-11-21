use std::error::Error;
use std::usize;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on port 3000....");
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Connection established!");

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let read_socket = socket.read(&mut buffer).await;
            let received_batch = match read_socket {
                Ok(n) => n,
                Err(arms) => 0,
            };
            println!("{:?}", String::from_utf8_lossy(&buffer[..received_batch]));
            socket
                .write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\nHello from Rust!")
                .await
                .unwrap();
        });
    }

    Ok(())
}
