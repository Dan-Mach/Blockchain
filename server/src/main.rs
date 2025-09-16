// server/src/main.rs

use blockchain::{Runtime, support};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instantiate your blockchain runtime and wrap it in a thread-safe container
    let runtime = Arc::new(Mutex::new(Runtime::new()));
    
    // Bind the server to a local address
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

        // Deserialize the received message as an Extrinsic
        let extrinsic: support::Extrinsic<blockchain::types::AccountId, blockchain::RuntimeCall> = 
            serde_json::from_slice(&buffer[..bytes_read])?;

        // Lock the runtime to process the extrinsic
        let mut runtime_guard = runtime.lock().unwrap();

        // Dispatch the extrinsic to your blockchain runtime
        let dispatch_result = runtime_guard.dispatch(extrinsic.caller, extrinsic.call);

        // Send a response back to the client
        let response = serde_json::to_string(&dispatch_result)?;
        stream.write_all(response.as_bytes()).await?;
    }
    Ok(())
}