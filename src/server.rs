use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::commands::{ircvers, mode, nick};
use crate::connection::Connection;
use crate::user::{User, UserList};

pub async fn handle_connection(
    mut socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    users: UserList,
) {
    println!("Handling connection from: {}", addr);

    let mut user = User {
        addr,
        nickname: None,
        connection: Connection::new(
            socket,
        ),
    };

    // Add the user to the global list
    {
        let mut user_list = users.write().await;
        user_list.insert(addr, &user);
        println!("User {} added", addr);
    }

    let mut buffer = [0; 1024];

    loop {
        // Read data from the socket
        match user.connection.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("Connection from {} closed", addr);
                break; // Connection closed gracefully
            }
            Ok(n) => {
                // Convert buffer to a string
                let newBuffer = buffer[..n].to_vec();
                let message = String::from_utf8_lossy(&newBuffer);
                    println!("Received message from {}: {}", addr, message);

                    // Split the message by spaces
                    let parts: Vec<&str> = message.split_whitespace().collect();

                    // Print the split parts (for now, just display them)
                    println!("Split parts: {:?}", parts);

                    // Example: Handle specific commands (you can implement actual logic here)
                    if let Some(command) = parts.get(0) {
                        match command.to_lowercase().as_str() {
                            "mode" => {
                                mode::execute(user).await;
                            }
                            "quit" => {
                                println!("Client {} requested to quit", addr);
                                break; // Exit the loop and close the connection
                            }
                            "nick" => {
                                nick::execute(&mut socket, &addr, &parts, &users).await;
                                break;
                            }
                            "ircvers" => {
                                ircvers::execute(&mut socket, &addr).await;
                            }
                            _ => {
                                println!("Unknown command from {}: {}", addr, command);
                            }
                        }
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
