use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::handler::command::help;

pub fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            let _ = stream.write_all(&mut buffer[0..size]);
            let client_sysinfo = String::from_utf8_lossy(&mut buffer[..size]);
            println!("Client sysinfo: {}", client_sysinfo);
            true
        },
        Err(_) => {
            println!(
                "An error occured, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn server() {
    let mut command = String::new();

    loop {
        print!("C2 => ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut command).expect("Failed to read command");
        command = command.trim_end().to_owned();

        if command == "help" {
            help();
        }

        if command == "exit" {

        }
    }
}

