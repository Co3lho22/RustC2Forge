use std::io::{Read, Write, Result};
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("192.168.1.70:7878")?;

    let message = "This is the windows 11 machine";
    stream.write_all(message.as_bytes())?;

    let mut response = Vec::new();
    stream.read(&mut response)?;

    let response_str = String::from_utf8_lossy(&response);
    println!("Response: {}", response_str);

    Ok(())
}

