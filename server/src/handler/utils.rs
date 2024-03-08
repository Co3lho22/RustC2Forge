use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::config::{ClientConfig, ClientDetails, ClientMap};
use crate::handler::command::help;

pub fn handle_client(mut stream: TcpStream, client_map: ClientMap){
    let peer_addr = stream.peer_addr().unwrap().to_string();
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(&stream);

    match reader.read_until(b'\n', &mut buffer){
        Ok(_) => {
            let config: Result<ClientConfig, serde_json::Error>
                = serde_json::from_slice(&buffer);

            match config {
                Ok(config) => {
                    println!("Receive config from {}: {:?}", peer_addr, config);
                    let mut map = client_map.lock().unwrap();
                    map.insert(peer_addr.clone(), ClientDetails {
                        config,
                        last_command: None,
                    });
                },
                Err(e) => {
                    println!("[E] Failed to deserialiy Client Config
                             from {}: {}", peer_addr, e)
                },
            }
        },
        Err(e) => println!("[E] Failed to read from {}: {}", peer_addr, e),
    }
}

pub fn server(server_client_map: Arc<Mutex<HashMap<String, ClientDetails>>>) {
    let mut command = String::new();

    loop {
        print!("C2 => ");
        io::stdout().flush().unwrap();

        command.clear();
        io::stdin().read_line(&mut command).expect("Failed to read command");
        command = command.trim_end().to_owned();

        println!("This was the command entered: {}", command);
        if command == "help" {
            help();
        }

        if command == "exit" {
            println!("Entered the command Exit");
            process::exit(0); // 0 -> success exit code
        }
    }
}

