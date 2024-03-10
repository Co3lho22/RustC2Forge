use crate::worker::sys_info::{get_network_info, get_cpu_arch, get_os};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub arch: String,
    pub network_info: Vec<(String, String)>,
    pub os: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            arch: get_cpu_arch(),
            network_info: get_network_info(),
            os: get_os(),
        }
    }

    pub fn get_arch(&self) -> &String {
        &self.arch
    }

    pub fn get_network_info(&self) -> &Vec<(String, String)> {
        &self.network_info
    }

    pub fn get_os(&self) -> &String {
        &self.os
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    name: String,
    output: Option<String>,
}

impl Command {
    pub fn new(name: &String) -> Self {
        Command {
            name: name.clone(),
            output: None,
        }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

