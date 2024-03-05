struct Config {
    pub arch: String,
    pub network_info: Vec<(String, String)>,
    pub os: String,
}

impl Config {
    pub fn new(
        arch: String,
        network_info:Vec<(String, String)>,
        os: String,
    ) -> Self {
        Self {
            arch,
            network_info,
            os,
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
}

