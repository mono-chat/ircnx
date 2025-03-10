use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Connection {
    pub socket: tokio::net::TcpStream,
    pub addr: std::net::SocketAddr,
}

impl Connection {
    pub fn new(socket: tokio::net::TcpStream) -> Self {
        let addr = socket.peer_addr().expect("Failed to get peer address");
        Connection { socket, addr }
    }

    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, tokio::io::Error> {
        self.socket.read(buffer).await
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), tokio::io::Error> {
        match self.socket.write_all(data).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to send response to socket {}: {}", self.addr, e);
                Err(e)
            }
        }
    }
}
