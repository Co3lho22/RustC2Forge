pub struct Config {
    pub server_ip: String,
    pub server_port: String,
    pub server_mac: String,
    pub server_os: String,
}

impl Config {
    pub fn new(
        server_ip: String,
        server_port: String,
        server_mac: String,
        server_os: String,
    ) -> Self {
        Self {
            server_ip,
            server_port,
            server_mac,
            server_os,
        }
    }
}

