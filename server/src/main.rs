use std::io::Write;
use std::net::TcpListener;
use std::{io, thread};
use crate::handler::utils::{handle_client, listen_for_heartbeats, monitor_heartbeats, server};
use crate::config::ClientManager;

mod handler;
mod config;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:49151").unwrap();
    println!("Server listening on port 49151");
    io::stdout().flush().unwrap();

    let client_manager: ClientManager = ClientManager::new();

    let server_client_manager_clone = client_manager.clone();
    thread::spawn(|| server(server_client_manager_clone));

    let heartbeat_client_manager_clone = client_manager.clone();
    thread::spawn(move || {
        monitor_heartbeats(heartbeat_client_manager_clone);
    });
    // thread to delete ClientManager elements that are no longer connected

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_manager_clone = client_manager.clone();
                let client_stream_1 = stream.try_clone().unwrap();
                thread::spawn(move || {
                    handle_client(client_stream_1, client_manager_clone);
                });


                let client_stream_2 = stream.try_clone().unwrap();
                let listen_heartbeats_client_manager_clone = client_manager.clone();
                thread::spawn(move || {
                    listen_for_heartbeats(client_stream_2, listen_heartbeats_client_manager_clone)
                });
                // thread to listen for heart beats
            },
            Err(e) => {
                println!("[E] Error while listening for new connections: {}", e);
            }
        }
    }
}

