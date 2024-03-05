use std::env;

pub fn get_os() -> String {
    let os = env::consts::OS;
    os.to_string()
}

