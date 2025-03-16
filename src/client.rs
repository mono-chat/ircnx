pub async fn handle_connection(socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    println!("Handling connection from: {}", addr);
    println!("Finished handling connection from: {}", addr);
}
