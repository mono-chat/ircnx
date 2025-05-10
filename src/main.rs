mod config;
mod parser;
mod transport;

use std::thread;

use tokio_stream::StreamExt;

use crate::transport::irc::IrcListener;

// The regex pattern for parsing IRC messages
const REGEX: &str = r"^ *(?:@(?<tags>[^ ]+) +)?(?::(?<prefix>[^ ]+) +)?(?<command>[^ ]+)(?: +(?<middle>[^: ][^ ]*(?: +[^: ][^ ]*)*))?(?: +:(?<trailing>.*))? *$";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load our configuration
    let settings = config::load().expect("Failed to load configuration");

    if settings.listen.is_empty() {
        // This shouldn't happen
        eprintln!("No listen configuration found");
        std::process::exit(1);
    }

    let mut handles = Vec::new();

    for listen in settings.listen {
        let listener = IrcListener::bind(((&listen.hostname).as_str(), listen.port))
            .await
            .expect("Failed to bind to address");

        let handle = tokio::spawn(async move {
            let mut incoming = listener.incoming();
            while let Some(stream) = incoming.next().await {
                match stream {
                    Ok(mut irc_stream) => {
                        println!("New connection!");

                        // Spawn a new task for each connection
                        tokio::spawn(async move {
                            loop {
                                match irc_stream.read().await {
                                    Ok(message) => {
                                        println!(
                                            "Parsed message: {:?}",
                                            parser::parse_irc_message(&message)
                                        );
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
                        });
                    }
                    Err(e) => eprintln!("Connection error: {}", e),
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all listener tasks to finish
    for handle in handles {
        handle.await.expect("Task panicked");
    }

    Ok(())
}