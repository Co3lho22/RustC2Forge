use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream){
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

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

