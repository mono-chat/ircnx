mod config;
mod transport;

use std::thread;

use crate::transport::irc::IrcListener;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load our configuration
    let settings = config::load().expect("Failed to load configuration");

    if settings.listen.is_empty() { // This shouldn't happen
        eprintln!("No listen configuration found");
        std::process::exit(1);
    }

    let mut handles = Vec::new();

    for listen in settings.listen {
        let listener = IrcListener::bind(((&listen.hostname).as_str(), listen.port))
            .expect("Failed to bind to address");

        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut irc_stream) => {
                        println!("New connection!");
                        loop {
                            match irc_stream.read() {
                                Ok(message) => {
                                    println!("Received IRC message: {}", message);
                                }
                                Err(e) => {
                                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                                        println!("Connection closed");
                                        break; // Exit the loop if the connection is closed
                                    } else {
                                        eprintln!("Error reading IRC message: {}", e);
                                        break; // Exit the loop if there's an error
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Connection error: {}", e),
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())
}
