use std::env;
use get_if_addrs::{get_if_addrs, IfAddr};

/// Returns the operating system of the client.
///
/// Utilizes the `env::consts::OS` constant to determine the operating system.
/// This function abstracts the process of accessing environment constants to
/// retrieve the OS name.
///
/// # Returns
///
/// Returns a `String` representing the name of the operating system.
pub fn get_os() -> String {
    let os = env::consts::OS;
    os.to_string()
}

/// Retrieves network interface information of the client.
///
/// Iterates over all network interfaces available on the client machine,
/// collecting their names and associated IP addresses. Supports both IPv4
/// and IPv6 addresses.
///
/// # Returns
///
/// Returns a vector of tuples, each containing the name of the network
/// interface and its IP address as strings. If an error occurs while retrieving
/// network interfaces, it logs the error and returns an empty vector.
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

/// Returns the CPU architecture of the client.
///
/// Utilizes the `env::consts::ARCH` constant to determine the CPU architecture.
/// This function simplifies the process of identifying the architecture of the
/// client's CPU.
///
/// # Returns
///
/// Returns a `String` representing the CPU architecture.
pub fn get_cpu_arch() -> String {
    let cpu_arch = env::consts::ARCH;
    cpu_arch.to_string()
}

