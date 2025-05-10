use tokio_stream::{wrappers::TcpListenerStream, Stream};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

pub struct IrcListener {
    listener: TcpListener,
}

impl IrcListener {
    pub async fn bind(addr: (&str, u16)) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr).await?;
        Ok(IrcListener { listener })
    }

    pub fn incoming(self) -> impl Stream<Item = Result<IrcStream, std::io::Error>> {
        TcpListenerStream::new(self.listener).map(|result| {
            result.map(|tcp_stream| IrcStream::new(tcp_stream))
        })
    }
}

pub struct IrcStream {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: u32,
    is_closed: bool,
}

const MAX_MESSAGE_SIZE: usize = 8703;

impl IrcStream {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::new(),
            cursor: 0,
            is_closed: false,
        }
    }

    pub async fn close(&mut self) -> std::io::Result<()> {
        self.is_closed = true;
        self.stream.shutdown().await
    }

    pub async fn read(&mut self) -> std::io::Result<String> {
        loop {
            if self.is_closed {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Connection closed",
                ));
            }

            if let Some(pos) = self.buffer.iter().position(|&b| b == b'\r' || b == b'\n') {
                let message = String::from_utf8_lossy(&self.buffer[..pos]).to_string();
                self.buffer.drain(..=pos);
                if !message.is_empty() {
                    return Ok(message);
                }
            }

            let mut temp_buf = [0; MAX_MESSAGE_SIZE];
            let bytes_read = self.stream.read(&mut temp_buf).await?; // Use async read

            if bytes_read == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Connection closed",
                ));
            }

            self.buffer.extend_from_slice(&temp_buf[..bytes_read]);
            if self.buffer.len() >= MAX_MESSAGE_SIZE * 2 {
                self.buffer.clear();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Message too large",
                ));
            }
        }
    }

    pub async fn write(&mut self, message: &str) -> std::io::Result<()> {
        let message = format!("{}\r\n", message);
        self.stream.write_all(message.as_bytes()).await // Use async write
    }
}