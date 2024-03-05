use std::io::{self, Read, Write, Result};
use std::net::TcpStream;

mod config;
mod worker;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("192.168.1.70:7878")?;
    println!("Connect to the server.");

    loop {
        let mut input = String::new();
        println!("Enter a message to send to the server:");
        io::stdin().read_line(&mut input)?;
        if input.trim() == "exit" {
            println!("Exiting.");
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buffer = vec![0; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server closed the connection.");
                break;
            },
            Ok(n) => {
                let response_str = String::from_utf8_lossy(&buffer[..n]);
                println!("Response from server: {}", response_str);
            },
            Err(e) => {
                println!("An error occured: {}", e);
                break;
            }
        }
    }

    Ok(())
}

