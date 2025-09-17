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
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");
    loop {
        // Accept incoming connections
        let (socket, _) = listener.accept().await?;
        // Clone the Arc to give each new task access to the shared runtime
        let runtime_clone = Arc::clone(&runtime);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, runtime_clone).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, runtime: Arc<Mutex<Runtime>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            // Connection closed
            break;
        }
    }
    Ok(())
}