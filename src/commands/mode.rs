use crate::user::User;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn execute(user: &mut User) {
    println!("Handling 'mode' command from {}", user.addr);

    let response = ":DefaultServerName 800 * 0 0 GateKeeper,Ntlm,ANON 1024 *\n";
    if let Err(e) = user.connection.write_all(response.as_bytes()).await {
        eprintln!("Failed to send response to socket {}: {}", user.addr, e);
    }
}
