use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::net::SocketAddr;
use crate::user::UserList;

pub async fn execute(socket: &mut TcpStream, addr: &SocketAddr, parts: &[&str], users: &UserList) {
    if let Some(nickname) = parts.get(1) {
        {
            let mut user_list = users.write().await;
            if let Some(user) = user_list.get_mut(&addr) {
                user.nickname = Some(nickname.to_string());
                println!("User {} set their nickname to {}", addr, nickname);
            }
        }
        let nick_ack_message = format!("Nickname set to: {}\n", nickname);
        if let Err(e) = socket.write_all(nick_ack_message.as_bytes()).await {
            eprintln!("Failed to send nickname acknowledgment to {}: {}", addr, e);
        }
    } else {
        let error_message = "Error: No nickname provided. Usage: nick <nickname>\n";
        if let Err(e) = socket.write_all(error_message.as_bytes()).await {
            eprintln!("Failed to send error to {}: {}", addr, e);
        }
    }
}

