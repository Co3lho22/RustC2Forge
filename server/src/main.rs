use std::io::Write;
use std::net::TcpListener;
use std::{io, thread};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::handler::utils::{server, handle_client};
use crate::config::{ClientMap, ClientDetails};

mod handler;
mod config;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");
    io::stdout().flush().unwrap();

    let client_map: ClientMap = Arc::new(Mutex::new(
            HashMap::<String, ClientDetails>::new()));

    let server_client_map = Arc::clone(&client_map);
    thread::spawn(|| server(server_client_map)).join().unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_map_clone = Arc::clone(&client_map);
                thread::spawn(move || {
                    handle_client(stream, client_map_clone);
                });
            },
            Err(e) => {
                println!("[E] Error while listening for new connections: {}", e);
            }
        }
    }
}

