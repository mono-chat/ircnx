use crate::listener::handle_connection;
use tokio::net::TcpListener;

mod connection;
mod listener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname: &str = "0.0.0.0";
    let port = 6667;
    let local_addr = format!("{}:{}", hostname, port);

    // Bind the server to a local address
    let listener = TcpListener::bind(&local_addr).await?;
    println!("Server is running on {}", &local_addr);

    loop {
        // Accept incoming connections
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("New connection from: {}", addr);
                // Spawn a task to handle the connection
                tokio::spawn(async move {
                    handle_connection(socket, addr).await;
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
