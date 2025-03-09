use crate::server::handle_connection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

mod commands;
mod connection;
mod server;
mod user;

type UserList = Arc<RwLock<HashMap<std::net::SocketAddr, user::User>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname: &str = "metal";
    let port = 6667;
    let local_addr = format!("{}:{}", hostname, port);

    // Shared state for connected users
    let users: UserList = Arc::new(RwLock::new(HashMap::new()));

    // Bind the server to a local address
    let listener = TcpListener::bind(&local_addr).await?;
    println!("Server is running on {}", &local_addr);

    loop {
        // Accept incoming connections
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("New connection from: {}", addr);
                let users = Arc::clone(&users);
                // Spawn a task to handle the connection
                tokio::spawn(async move {
                    handle_connection(socket, addr, users).await;
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
