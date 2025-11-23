use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::usize;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load the env
    dotenvy::dotenv()?;

    // connect to the DB;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    println!("✅ Connection to Postgres successful!");
    initiate_users_table(&pool).await?;

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on port 3000....");
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Connection established!");

        tokio::spawn(async move {
            let mut buffer = [0; 4096];
            match socket.read(&mut buffer).await {
                Ok(0) => return,
                Ok(n) => {
                    let request = String::from_utf8_lossy(&buffer[..n]).to_string();
                    println!("Request => {}", request);
                    let (status_line, filename) = if request.starts_with("GET / ") {
                        println!("Routing to the root resource...");
                        ("HTTP/1.1 200 OK", "hello.html")
                    } else if request.starts_with("GET /login") {
                        println!("Routing to the login resource...");
                        ("HTTP/1.1 200 OK", "login.html")
                    } else {
                        println!("404 Not found");
                        ("HTTP/1.1 404 NOT FOUND", "404.html")
                    };

                    let contents = fs::read_to_string(filename).await.unwrap();
                    let response =
                        format!("{}\r\nConnection: close\r\n\r\n{}", status_line, contents);
                    socket.write_all(response.as_bytes()).await.unwrap();
                }
                Err(e) => println!("failed to read from socket; err = {:?}", e),
            };
        });
    }

    Ok(())
}

async fn initiate_users_table(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await;
    println!("User table created...");

    sqlx::query(
        r#"
        INSERT INTO users (id, name, email)
        VALUES (1, 'Fathan', 'fathan@rust.com')
        ON CONFLICT
            DO NOTHING
        "#,
    )
    .execute(pool)
    .await
    .inspect(|_| println!("Dummy user created..."));

    Ok(())
}
