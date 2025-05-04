// use std::io;
// use std::net::TcpListener;
// use std::thread;

// pub fn bind_tcp(address: String, port: u16) -> io::Result<TcpListener> {
//     let listener = TcpListener::bind((address, port))?;
//     println!("Bound TCP listener on {}", listener.local_addr()?);
//     Ok(listener)
// }

// pub fn handle_incoming_tcp(listener: TcpListener, handler: &'static (dyn Fn(io::Result<std::net::TcpStream>, u16) -> io::Result<()> + Send + Sync)) {
//     let port = listener.local_addr().unwrap().port(); // Safe to unwrap as we just bound it
//     thread::spawn(move || {
//         println!("Listening for TCP on port {}", port);
//         for stream in listener.incoming() {
//             if let Err(e) = handler(stream, port) {
//                 eprintln!("Error handling TCP connection on port {}: {}", port, e);
//             }
//         }
//         println!("TCP listener on port {} stopped.", port);
//     });
// }



use std::net::{ TcpListener, TcpStream, ToSocketAddrs };
use std::io::{Read, Result};

pub struct IrcListener {
    listener: TcpListener,
}

impl IrcListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }

    fn accept(&self) -> std::io::Result<IrcStream> {
        let (stream, _) = self.listener.accept()?;
        Ok(IrcStream::new(stream))
    }

    pub fn incoming(&self) -> impl Iterator<Item = std::io::Result<IrcStream>> + '_ {
        self.listener.incoming().map(|res| res.map(IrcStream::new))
    }
}

pub struct IrcStream {
    stream: TcpStream,
    buffer: Vec<u8>,
}

const MAX_MESSAGE_SIZE: usize = 8703; // 512 bytes for IRC messages

impl IrcStream {
    fn new(stream: TcpStream) -> Self {
        Self { stream, buffer: Vec::new() }
    }

    // A message size of 8703 covers all IRCv3 messages. We should reduce this to 4610 bytes (IRCv3 client with Message Tags) and 512 bytes (IRC/IRCX client default) depending on client capabilities. 8703 is only required for IRCv3 server-to-server messages.
    // We also need to handle the case where the message is larger than 8703 bytes. This is not a common case, but we should handle it gracefully.
    pub fn read(&mut self) -> Result<String> {
        let mut temp_buf = [0; MAX_MESSAGE_SIZE]; // Temporary buffer to read data
        let bytes_read = self.stream.read(&mut temp_buf)?;

        if bytes_read == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Connection closed"));
        }

        // Append new data to internal buffer
        self.buffer.extend_from_slice(&temp_buf[..bytes_read]);

        // Check for a complete IRC message
        if let Some(pos) = self.buffer.iter().position(|&b| b == b'\r' || b == b'\n') {
            let message = String::from_utf8_lossy(&self.buffer[..pos]).to_string();
            self.buffer.drain(..pos + 1);
            return Ok(message);
        }

        // If no complete message is found, continue waiting for more data
        Err(std::io::Error::new(std::io::ErrorKind::WouldBlock, "Partial message, waiting for more data"))
    }
}