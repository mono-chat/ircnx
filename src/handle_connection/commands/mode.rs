use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::net::SocketAddr;

pub async fn execute(socket: &mut TcpStream, addr: &SocketAddr) {
    println!("Handling 'mode' command from {}", addr);

    let response = ":DefaultServerName 800 * 0 0 GateKeeper,Ntlm,ANON 1024 *\n";
    if let Err(e) = socket.write_all(response.as_bytes()).await {
        eprintln!(
            "Failed to send response to socket {}: {}",
            addr, e
        );
    }
}
