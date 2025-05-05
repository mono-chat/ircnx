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

use std::char::MAX;
use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

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
    cursor: u32,
}

const MAX_MESSAGE_SIZE: usize = 8703; // 512 bytes for IRC messages

impl IrcStream {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::new(),
            cursor: 0,
        }
    }

    // A message size of 8703 covers all IRCv3 messages. We should reduce this to 4610 bytes (IRCv3 client with Message Tags) and 512 bytes (IRC/IRCX client default) depending on client capabilities. 8703 is only required for IRCv3 server-to-server messages.
    // We also need to handle the case where the message is larger than 8703 bytes. This is not a common case, but we should handle it gracefully.

    // TODO: Needs improvements. We always read from the beginning of the buffer, which is wasteful. We should keep track of the cursor position and only read from the buffer starting from the cursor position.
    pub fn read(&mut self) -> std::io::Result<String> {
        loop {
            // Check if there's a complete message in the buffer
            if let Some(pos) = self.buffer.iter().position(|&b| b == b'\r' || b == b'\n') {
                // Extract message
                let message = String::from_utf8_lossy(&self.buffer[..pos]).to_string(); // This is a UTF-8 conversion. Not sure we want this.
                
                // Remove processed data from buffer (including delimiter)
                self.buffer.drain(..=pos);
                
                if message.len() > 0 {
                    return Ok(message); // Return only when a message is ready
                }
                else {
                    // If the message is empty, continue to read more data (wasteful, as we start reading from the beginning of the buffer again)
                    continue;
                }
            }

            // Read more data from the socket (blocking until data arrives)
            let mut temp_buf = [0; MAX_MESSAGE_SIZE];
            let bytes_read = self.stream.read(&mut temp_buf)?;

            if bytes_read == 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Connection closed"));
            }

            // Append new data to buffer
            self.buffer.extend_from_slice(&temp_buf[..bytes_read]);
            if (self.buffer.len() >= MAX_MESSAGE_SIZE * 2) { // We shouldn't ever reach twice the max message size, since the first message should have been read if it's max size.
                self.buffer.clear(); // Clear the buffer if we exceed the max message size
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Message too large"));
            }
        }
    }

}
