use std::io::{self, Write, Read, BufRead, BufReader};
use crate::worker::network::connect_server;
use crate::config::Config;

mod config;
mod worker;

fn main() -> io::Result<()> {
    let ip = String::from("192.168.1.70");
    let port = String::from("7878");
    let mut stream = connect_server(ip.clone(), port.clone())?;
    println!("[I] Connected to server {}:{}.", ip, port);

    // Create Config object and serialize it
    let config = Config::new();
    let config_json = config.to_json().expect("Failed to serialize Config");
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

        let command = String::from_utf8_lossy(&buffer);
        println!("[I] Received command: {}", command.trim_end());

    }

    Ok(())
}

