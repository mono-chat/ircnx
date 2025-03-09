use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::handle_connection::commands::mode;
use crate::user::{User, UserList};
mod commands {
    pub mod mode;
}

pub async fn handle_connection(
    mut socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    users: UserList,
) {
    println!("Handling connection from: {}", addr);

    // Add the user to the global list
    {
        let mut user_list = users.write().await;
        user_list.insert(addr, User { addr });
        println!("User {} added", addr);
    }

    let mut buffer = [0; 1024];

    loop {
        // Read data from the socket
        match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("Connection from {} closed", addr);
                break; // Connection closed gracefully
            }
            Ok(n) => {
                // Convert buffer to a string
                if let Ok(message) = String::from_utf8(buffer[..n].to_vec()) {
                    println!("Received message from {}: {}", addr, message);

                    // Split the message by spaces
                    let parts: Vec<&str> = message.split_whitespace().collect();

                    // Print the split parts (for now, just display them)
                    println!("Split parts: {:?}", parts);

                    // Echo the data back to the client
                    if let Err(e) = socket.write_all(message.as_bytes()).await {
                        eprintln!("Failed to write to socket {}: {}", addr, e);
                        break;
                    }

                    // Example: Handle specific commands (you can implement actual logic here)
                    if let Some(command) = parts.get(0) {
                        match command.to_lowercase().as_str() {
                            "mode" => {
                                // Call the hello handler
                                mode::execute(&mut socket, &addr).await;
                            }
                            "quit" => {
                                println!("Client {} requested to quit", addr);
                                break; // Exit the loop and close the connection
                            }
                            _ => {
                                println!("Unknown command from {}: {}", addr, command);
                            }
                        }
                    }
                } else {
                    // Invalid UTF-8 data received
                    eprintln!("Received invalid UTF-8 data from {}", addr);
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket {}: {}", addr, e);
                break;
            }
        }
    }


    // Remove the user from the global list
    {
        let mut user_list = users.write().await;
        user_list.remove(&addr);
        println!("User {} removed", addr);
    }

    println!("Finished handling connection from: {}", addr);
}
