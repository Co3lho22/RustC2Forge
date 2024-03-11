use std::io::{self, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::config::ClientManager;

/// Monitors heartbeats from clients and removes inactive clients.
///
/// This function continuously checks for clients that have not sent a heartbeat
/// within a predefined threshold and removes them from the `ClientManager`. It
/// runs in an infinite loop, periodically sleeping for a set duration before
/// checking heartbeats again.
///
/// # Parameters
///
/// * `heartbeat_client_manager`: An instance of `ClientManager` responsible
/// for managing client heartbeats.
pub fn monitor_heartbeats(heartbeat_client_manager: ClientManager) {
    loop {
        let clients_to_remove = heartbeat_client_manager.check_heartbeats();
        for ip in clients_to_remove {
            println!("Heart beats to remove ip {}", ip);
            heartbeat_client_manager.remove_client(&ip);
            println!("[I] Client {} removed due to missing heartbeats", ip);
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

/// Handles heartbeat signals from a specific client.
///
/// Listens for heartbeat messages from a client, updating the last heartbeat
/// timestamp each time a message is received. If the connection is closed or
/// an error occurs, it removes the client from the `ClientManager`.
///
/// # Parameters
///
/// * `stream`: The TCP stream associated with the client.
/// * `client_manager`: An instance of `ClientManager` for managing the client.
/// * `ip`: The IP address of the client as a String.
pub fn listen_for_heartbeats_aux(stream: TcpStream, client_manager: ClientManager, ip: String){
    let mut reader = BufReader::new(&stream);

    loop {
        let mut buffer = Vec::new();
        match reader.read_until(b'\n', &mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    client_manager.remove_client(&ip);
                    break;
                }

                let message = String::from_utf8_lossy(&buffer).trim().to_string();
                if message == "heartbeat" {
                    client_manager.update_heartbeat(&ip);
                }
            },
            Err(e) => {
                println!("[E] Error reading from: {}: {}", ip, e);
            }
        }
    }
}

/// Initiates a listening service for receiving heartbeat signals from clients.
///
/// Sets up a `TcpListener` to accept incoming TCP connections on a specified
/// port, each representing a heartbeat signal from a client. For each
/// connection, it spawns a new thread to handle the heartbeat signals using
/// `listen_for_heartbeats_aux`.
///
/// # Parameters
///
/// * `client_manager`: An instance of `ClientManager` to pass to the
/// heartbeat handler.
/// * `ip`: The IP address of the client to listen for heartbeats.
pub fn listen_for_heartbeats(client_manager: ClientManager, ip: String){
    let listener = TcpListener::bind("0.0.0.0:52222").unwrap();
    io::stdout().flush().unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let heartbeats_client_manager_clone = client_manager.clone();
                let client_ip = ip.clone();
                thread::spawn(move || {
                    listen_for_heartbeats_aux(stream,
                                              heartbeats_client_manager_clone, client_ip);
                });
            },
            Err(e) => {
                println!("[E] Error while listening for new heartbeat \
                         connections: {}", e);
            }
        }
    }
}

