use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            let _ = stream.write_all(&mut buffer[0..size]);
            let client_message = String::from_utf8_lossy(&mut buffer[..size]);
            println!("Client message: {}", client_message);
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

