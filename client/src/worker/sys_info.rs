use std::env;
use get_if_addrs::{get_if_addrs, IfAddr};

pub fn get_os() -> String {
    let os = env::consts::OS;
    os.to_string()
}

pub fn get_network_info() -> Vec<(String, String)>  {
    let mut interfaces = Vec::new();

    match get_if_addrs() {
        Ok(if_addrs) => {
            for iface in if_addrs {
                let name = iface.name;
                let ip = match iface.addr {
                    IfAddr::V4(ref addr) => addr.ip.to_string(),
                    IfAddr::V6(ref addr) => addr.ip.to_string(),
                };
                interfaces.push((name, ip));
            }
        },
        Err(e) => println!("Error retrieving network interfaces: {}", e),
    }
    interfaces
}

pub fn get_cpu_arch() -> String {
    let cpu_arch = env::consts::ARCH;
    cpu_arch.to_string()
}

