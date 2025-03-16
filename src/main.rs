pub mod channels;
mod config;
pub mod connections;

// We need to clean this up.
mod client;
use client::handle_connection;

use crate::channels::*;
use crate::config::*;
use crate::connections::*;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the configuration file
    let config = Config::load_from_file("config.toml")?;
    println!("Server Name: {}", config.server_name);

    // TODO: Loop through all listen addresses. For now, we're just using #1
    let addresses = &config.listen;
    listen_on_addresses(addresses).await?;

    Ok(())
}

// Todo: Move this to a separate file
async fn listen_on_addresses(addresses: &Vec<ListenAddr>) -> Result<(), Box<dyn std::error::Error>> {
    let mut listeners = Vec::new();

    for address in addresses {
        let addr = format!("{}:{}", address.hostname, address.port);
        let listener = TcpListener::bind(&addr).await?;
        println!("Listening on {}", addr);
        listeners.push(listener);
    }

    let mut tasks = Vec::new();

    for listener in listeners {
        let task = tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((socket, addr)) => {
                        println!("New connection from: {}", addr);
                        tokio::spawn(async move {
                            handle_connection(socket, addr).await;
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}