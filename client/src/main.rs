use serde_json::Result;
use std::net::TcpStream;
use crate::worker::network::connect_server;
use crate::config::Config;

mod config;
mod worker;

fn main() -> Result<()> {
    let ip = String::from("192.168.1.70");
    let port = String::from("7878");
    let stream = connect_server(ip, port);
    println!("[I] Connected to server {}:{}.", ip, port);

    // Create Config object and serialize it
    let config = Config::new();
    let config_json = config.to_json().expect("Failed to serialize Config");
    stream.write_all(config_json.as_bytes())?;

    Ok(())
}

