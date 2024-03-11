use std::io::{self, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::config::ClientManager;

/// Monitors heartbeats from clients, removing those that have not sent a
/// heartbeat within a threshold.
///
/// This function runs in a loop, periodically checking for clients that have
/// not sent heartbeats within a predetermined time frame and removing them
/// from the client manager.
///
/// # Parameters
///
/// * `heartbeat_client_manager`: An instance of `ClientManager` used for
/// managing client heartbeats.
pub fn monitor_heartbeats(heartbeat_client_manager: ClientManager) {
    loop {
        let clients_to_remove = heartbeat_client_manager.check_heartbeats();
        for ip in clients_to_remove {
            heartbeat_client_manager.remove_client(&ip);
            println!("[I] Client {} removed due to missing heartbeats", ip);
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

/// Listens for heartbeat messages from a specific client.
///
/// This function continuously reads messages from a client's TCP stream,
/// updating the client's last
/// heartbeat timestamp on receipt of a heartbeat message. If the connection
/// is closed, it removes the client from the client manager.
///
/// # Parameters
///
/// * `stream`: The TCP stream associated with the client.
/// * `client_manager`: An instance of `ClientManager` used for managing client
/// heartbeats and removal.
pub fn listen_for_heartbeats_aux(stream: TcpStream, client_manager: ClientManager){
    let ip = stream.peer_addr().unwrap().to_string();

    let mut reader = BufReader::new(&stream);

    loop {
        let mut buffer = Vec::new();
        match reader.read_until(b'\n', &mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    println!("[I] Connection closed by client: {}", ip);
                    client_manager.remove_client(&ip);
                    break;
                }

                let message = String::from_utf8_lossy(&buffer).trim().to_string();
                if message == "heartbeat" {
                    println!("[I] Heartbeat received from {}", ip);
                    client_manager.update_heartbeat(&ip);
                }
            },
            Err(e) => {
                println!("[E] Error reading from: {}: {}", ip, e);
            }
        }
    }
}


pub fn listen_for_heartbeats(client_manager: ClientManager){
    let listener = TcpListener::bind("0.0.0.0:52222").unwrap();
    println!("Listening for Heartbeat on port 52222");
    io::stdout().flush().unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let heartbeats_client_manager_clone = client_manager.clone();
                thread::spawn(move || {
                    listen_for_heartbeats_aux(stream,
                                              heartbeats_client_manager_clone);
                });
            },
            Err(e) => {
                println!("[E] Error while listening for new heartbeat \
                         connections: {}", e);
            }
        }
    }
}

