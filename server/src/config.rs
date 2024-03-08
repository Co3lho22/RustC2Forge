use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub arch: String,
    pub network_info: Vec<(String, String)>,
    pub os: String,
}

#[derive(Debug)]
pub struct ClientDetails {
    pub config: ClientConfig,
    pub last_command: Option<String>,
}

pub type ClientMap = Arc<Mutex<HashMap<String, ClientDetails>>>;

pub struct ClientManager {
    clients: ClientMap,
}

impl ClientManager {
    pub fn new(&self) -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::<String, ClientDetails>::new()))
        }
    }

    pub fn add_client(&self, ip: String, details: ClientDetails){
        let mut clients = self.clients.lock().unwrap();
        clients.insert(ip, details);
    }

    pub fn remove_client(&self, ip: &str) {
        let mut clients = self.clients.lock().unwrap();
        clients.remove(ip);
    }

    pub fn list_clients(&self) -> Vec<String> {
        let clients = self.clients.lock().unwrap();
        clients.keys().cloned().collect()
    }
}

