use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::net::SocketAddr;

pub async fn execute(socket: &mut TcpStream, addr: &SocketAddr) {
    println!("Client {} requested the IRC version", addr);
    let version_message = "IRC Server Version: 1.0.0\n";
    if let Err(e) = socket.write_all(version_message.as_bytes()).await {
        eprintln!("Failed to send version to {}: {}", addr, e);
    }
}
