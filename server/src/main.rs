use std::io::Write;
use std::net::TcpListener;
use std::{io, thread};
use crate::handler::utils::{handle_client, server};
use crate::handler::sys_info::get_os;

mod handler;
mod config;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let server_os: String = get_os();
    println!("Server listening on port 7878 | server_os = {}", server_os);
    io::stdout().flush().unwrap();

    thread::spawn(|| server());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

