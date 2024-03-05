pub struct Config {
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
}

