use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::user::User;

pub async fn execute(user: &mut User) {
    println!("Client {} requested the IRC version", user.connection.addr);
    let version_message = "IRC Server Version: 1.0.0\n";
    if let Err(e) = user.connection.write(version_message.as_bytes()).await {
        eprintln!("Failed to send version to {}: {}", user.connection.addr, e);
    }
}
