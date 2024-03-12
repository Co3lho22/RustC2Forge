use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

/// Represents the configuration details of a client.
///
/// Includes architectural information, network interfaces, and operating system details.
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub arch: String,
    pub network_info: Vec<(String, String)>,
    pub os: String,
}

/// Stores detailed information about a client, including its configuration,
/// optional command to execute, and the last heartbeat received.
#[derive(Debug)]
pub struct ClientDetails {
    pub config: ClientConfig,
    pub command: Option<String>,
}


// Struct used to store and send the command to execute to the client
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCommand {
    pub name: String,
    pub output: Option<String>,
}

impl ClientCommand {
    pub fn new(cmd: &String) -> Self {
        ClientCommand {
            name: cmd.clone(),
            output: None,
        }
    }

    /// Serializes the `ClientCommand` to a JSON string.
    ///
    /// Returns the JSON string if successful, or an error if serialization fails.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}



pub type ClientMap = Arc<Mutex<HashMap<String, ClientDetails>>>;

/// Manages client connections, including adding, removing, and querying client details.
#[derive(Clone)]
pub struct ClientManager {
    pub clients: ClientMap,
}

impl ClientManager {
    pub fn new() -> Self {
        ClientManager {
            clients: Arc::new(Mutex::new(HashMap::<String, ClientDetails>::new()))
        }
    }

    /// Adds a client to the manager.
    ///
    /// `ip`: The IP address of the client as a String.
    /// `details`: The `ClientDetails` struct containing information about the client.
    pub fn add_client(&self, ip: String, details: ClientDetails){
        let mut clients = self.clients.lock().unwrap();
        clients.insert(ip, details);
    }

    /// Removes a client from the manager by IP address.
    ///
    /// `ip`: The IP address of the client to remove as a String.
    pub fn remove_client(&self, ip: &String) {
        let mut clients = self.clients.lock().unwrap();
        clients.remove(ip);
    }

    /// Lists all clients currently managed by the `ClientManager`.
    ///
    /// Returns a Vector of IP addresses as Strings.
    pub fn list_clients(&self) -> Vec<String> {
        let clients = self.clients.lock().unwrap();
        clients.keys().cloned().collect()
    }

    /// Retrieves the command for a given client by IP address.
    ///
    /// `ip`: The IP address of the client.
    ///
    /// Returns an Option containing the command as a String, or None if not found.
    pub fn get_command(&self, ip: &String) -> Option<String> {
        let clients = self.clients.lock().unwrap();
        clients.get(ip).and_then(|client_details| client_details.command.clone())
    }

    /// Updates the command for a given client.
    ///
    /// `ip`: The IP address of the client.
    /// `cmd`: The new command to set for the client.
    ///
    /// Returns the previous command as an Option, or None if the client was not found.
    pub fn update_command(&self, ip: &String, cmd: String) -> Option<String>{
        let mut clients = self.clients.lock().unwrap();
        let cmd: Option<String> = Some(cmd);
        if let Some(client_details) = clients.get_mut(ip) {
            client_details.command = cmd.clone();
            cmd
        } else {
            None
        }
    }

    /// Resets the command for a given client, effectively clearing it.
    ///
    /// `ip`: The IP address of the client.
    ///
    /// Returns `Ok(())` if successful, or an `Err` with a message if the client was not found.
    pub fn reset_command(&self, ip: &String) -> Result<(), String> {
        let mut clients = self.clients.lock().unwrap();
        if let Some(client_details) = clients.get_mut(ip) {
            client_details.command = None;
            Ok(())
        } else {
            Err(format!("Client with IP {} not found", ip))
        }
    }

    /// Checks if a client exists within the manager.
    ///
    /// `ip`: The IP address of the client.
    ///
    /// Returns `true` if the client exists, otherwise `false`.
    pub fn client_exists(&self, ip: &String) -> bool {
        let clients = self.clients.lock().unwrap();
        clients.contains_key(ip)
    }


}
