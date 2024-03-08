use std::io::Write;
use std::net::TcpListener;
use std::{io, thread};
use crate::handler::utils::{server, handle_client};
use crate::config::ClientManager;

mod handler;
mod config;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");
    io::stdout().flush().unwrap();

    let client_manager: ClientManager = ClientManager::new();

    let server_client_manager_clone = ClientManager::clone(&client_manager);
    thread::spawn(|| server(server_client_manager_clone));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_manager_clone = ClientManager::clone(&client_manager);
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

