use std::io;
use std::net::TcpStream;

pub fn connect_server(
    ip: String,
    port: String
    ) -> Result<TcpStream, io::Error> {

    let full_address = format!("{}:{}", ip, port);

    let stream = TcpStream::connect(full_address);
    stream
}

