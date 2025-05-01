mod config;

use std::net::{IpAddr, TcpListener};

fn main() {
    // Load our configuration
    let settings = config::load().expect("Failed to load configuration");

    if settings.listen.is_empty() {
        eprintln!("No listen configuration found");
        std::process::exit(1);
    }
    for listen in settings.listen {
        let listener = TcpListener::bind((listen.hostname.as_str(), listen.port))
            .expect("Failed to bind to address");

        let local_address = listener.local_addr().expect("Failed to get local address");
        let family = match local_address.ip() {
            IpAddr::V4(_) => "IPv4",
            IpAddr::V6(_) => "IPv6",
        };
        println!("Listening on: {} ({})", local_address.to_string(), family);
    }
}
