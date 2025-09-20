// server/src/main.rs
use blockchain::support::Dispatch;
use blockchain::{Runtime, support};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Arc::new(Mutex::new(Runtime::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    loop {
        let (socket, _) = listener.accept().await?;
        let runtime = Arc::clone(&runtime);
        tokio::spawn(async move {
            handle_connection(socket, runtime);
        });
    }
}

fn handle_connection(mut socket: TcpStream, runtime: Arc<Mutex<Runtime>>) {
    let mut buffer = [0; 1024];
    tokio::spawn(async move {
        match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return, // connection closed
            Ok(n) => {
                // For simplicity, always respond with index.html
                let response_body = std::fs::read_to_string("./assets/index.html").unwrap_or_else(|_| "<h1>File not found</h1>".to_string());
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                    response_body.len(),
                    response_body
                );
                let _ = socket.write_all(response.as_bytes()).await;
            }
            Err(_) => return,
        }
    });
    
}
